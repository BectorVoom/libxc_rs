//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1126/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1126(t41245: f64, t47372: f64, t626: f64, t11: f64, t625: f64, t47377: f64, t5063: f64, t1691: f64, t1642: f64, t47733: f64, t17900: f64, t30955: f64, t30957: f64, t30962: f64, t32373: f64, t32375: f64, t41888: f64, t41890: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t47928 = 64.0_f64 / 45.0_f64 * t41245;
    let t47929 = t626 * t47372;
    let t47931 = t11 * t625 * t47929;
    let t47940 = t5063 * t47377;
    let t47942 = t11 * t1691 * t47940;
    let t47944 = t1642 * t47733;
    let t47946 = t11 * t1691 * t47944;
    let t47948 = -0.35991666666666666667e-1_f64 * t47931 + t17900 + 0.17777777777777777778e-1_f64 * t41888 - 0.10666666666666666667e0_f64 * t41890 - 0.63985185185185185184e-1_f64 * t30955 - 0.95977777777777777776e-1_f64 * t30957 + 0.19195555555555555555e0_f64 * t30962 - 0.44444444444444444445e-1_f64 * t32373 - 0.14814814814814814815e-1_f64 * t32375 - 0.86380000000000000002e0_f64 * t47942 - 0.71983333333333333335e-1_f64 * t47946;
    (t47928, t47929, t47931, t47940, t47942, t47944, t47946, t47948)
}
