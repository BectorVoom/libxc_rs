//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1798/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1798<F: Float>(t12448: F, t12463: F, t1169: F, t1159: F, t3475: F, t426: F) -> (F, F, F, F) {
    let t12464 = t12448 + t12463;
    let t12465 = t12464 * t1169;
    let t12469 = F::cast_from(1.0_f64) / t3475 / t1159;
    let t12470 = t426 * t12469;
    (t12464, t12465, t12469, t12470)
}
