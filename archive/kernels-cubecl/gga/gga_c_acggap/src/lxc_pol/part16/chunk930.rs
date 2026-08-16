//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 930/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk930<F: Float>(t3919: F, t7948: F, t3035: F, t3923: F, t609: F, t30028: F, t315: F, t323: F, t3242: F, t7927: F, t872: F, t2130: F, t3874: F) -> (F, F, F, F, F, F) {
    let t32082 = t7948 * t3919;
    let t32091 = F::cast_from(0.39512695097613069591e1_f64) * t3035 * t609 * t3923;
    let t32092 = t315 * t30028;
    let t32109 = F::cast_from(0.19756347548806534796e1_f64) * t3242 * t609 * t323;
    let t32121 = t7927 * t872;
    let t32123 = t2130 * t3874;
    (t32082, t32091, t32092, t32109, t32121, t32123)
}
