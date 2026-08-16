//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2557/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2557(t55279: f64, t3115: f64, t42793: f64, t4911: f64, t11200: f64, t380: f64, t16088: f64, t3057: f64, t4930: f64, t1071: f64, t15669: f64, t12050: f64, t15907: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t55280 = 0.14291339372689912324e-3_f64 * t55279;
    let t55293 = t3115 * t42793 * t4911;
    let t55294 = 0.14291339372689912324e-3_f64 * t55293;
    let t55330 = t11200 * t380;
    let t55331 = t55330 * t16088;
    let t55413 = t3057 * t4930;
    let t55464 = t15669 * t1071;
    let t55499 = t15907 * t12050;
    (t55280, t55294, t55330, t55331, t55413, t55464, t55499)
}
