//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 527/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk527<F: Float>(t114: F, t2366: F, t655: F, t2335: F, t2336: F, t2341: F, t69: F) -> (F, F) {
    let t115 = F::cast_from(1.0_f64) < t114;
    let t2367 = t655 * t2366;
    let t2371 = piecewise3::<F>(t115, F::cast_from(0.0_f64), t2335 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t2336 + t69 * t2341 / F::cast_from(4.0_f64) - t69 * t2367 / F::cast_from(8.0_f64));
    (t2367, t2371)
}
