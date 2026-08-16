//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1303/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1303(t22674: f64, t28191: f64, t80681: f64, t28206: f64, t6883: f64, t28205: f64, t6897: f64, t22892: f64, t28209: f64, t225: f64, t28051: f64, t28117: f64, t81159: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t96848 = t80681 * t22674 * t28191;
    let t96868 = t6883 * t28206;
    let t96878 = t6897 * t22674 * t28205;
    let t96893 = t22892 * t22674 * t28209;
    let t96913 = t28051 * t225;
    let t96920 = t81159 * t28117;
    (t96848, t96868, t96878, t96893, t96913, t96920)
}
