//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 630/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk630<F: Float>(t3495: F, t439: F, t3356: F, t3413: F, t1178: F) -> (F, F, F, F, F) {
    let t3496 = t439 * t3495;
    let t3503 = F::cast_from(0.40256666666666666667e0_f64) * t3356;
    let t3510 = F::cast_from(0.137975e0_f64) * t3413;
    let t3519 = t1178 * t1178;
    let t3520 = F::cast_from(1.0_f64) / t3519;
    (t3496, t3503, t3510, t3519, t3520)
}
