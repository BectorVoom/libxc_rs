//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2193/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2193(t28200: f64, t6883: f64, t225: f64, t28053: f64, t6888: f64, t7691: f64, t90739: f64, t1375: f64, t1386: f64, t20025: f64, t2016: f64, t26224: f64, t26225: f64, t26366: f64, t3887: f64, t5210: f64, t5354: f64, t539: f64, t56422: f64, t568: f64, t6460: f64, t6992: f64, t7722: f64, t81399: f64, t93906: f64, t97468: f64) -> f64 {
    let t97750 = t6883 * t28200;
    let t97756 = t28053 * t225;
    let t97766 = t6888 * t90739 * t7691;
    let t97770 = 2.0_f64 * t1375 * t3887 * t6992 * t6460 - 0.19190897446562641759e-1_f64 * t97750 + t539 * t97468 * t568 + t93906 - 2.0_f64 * t56422 * t2016 - 2.0_f64 * t97756 * t1386 + 2.0_f64 * t5210 * t7722 * t568 - 6.0_f64 * t26224 * t26225 * t20025 - 0.3289868133696452873e-1_f64 * t97766 - t81399 - 2.0_f64 * t26366 * t5354;
    t97770
}
