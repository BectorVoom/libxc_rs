//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 545/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk545<F: Float>(t114: F, t1513: F, t655: F, t653: F, t69: F) -> (F, F) {
    let t115 = F::new(1.0) < t114;
    let t1514 = t655 * t1513;
    let t1518 = piecewise3::<F>(t115, F::new(0.0), -t653 - t69 * t1514 / F::new(8.0));
    (t1514, t1518)
}
