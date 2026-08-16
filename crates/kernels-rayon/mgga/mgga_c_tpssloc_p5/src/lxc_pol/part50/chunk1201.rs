//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1201/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1201(t10164: f64, t1052: f64, t113243: f64, t113278: f64, t113286: f64, t113313: f64, t14529: f64, t14555: f64, t1599: f64, t1603: f64, t1920: f64, t1956: f64, t225: f64, t23327: f64, t25705: f64, t25743: f64, t25750: f64, t25757: f64, t3026: f64, t30782: f64, t30843: f64, t30900: f64, t3169: f64, t3174: f64, t32913: f64, t32917: f64, t345: f64, t387: f64, t388: f64, t4552: f64, t4557: f64, t4664: f64, t4693: f64, t6687: f64, t6771: f64, t8391: f64, t8397: f64, t8406: f64, t8407: f64, t88050: f64, t88744: f64) -> f64 {
    let t119149 = 2.0_f64 * t14529 * t8397 + 2.0_f64 * t14555 * t8397 + t4552 * t8391 * t388 + t1603 * t30843 * t388 + 2.0_f64 * t3169 * t32913 - 2.0_f64 * t88744 * t1956 + 2.0_f64 * t1052 * t3174 * t8406 * t4693 + 0.10966227112321509577e-1_f64 * t113286 + 0.16449340668482264365e-1_f64 * t1920 * t345 * t25705 * t225 * t387 - t14555 * t8407 - 0.54831135561607547883e-2_f64 * t23327 * t113243 * t25750 - 0.54831135561607547883e-2_f64 * t23327 * t88050 * t30782 + 0.16449340668482264365e-1_f64 * t6687 * t1599 * t113278 + 4.0_f64 * t6771 * t25743 - 6.0_f64 * t25757 * t10164 * t8406 * t4664 - t4557 * t30900 + 4.0_f64 * t3026 * t32917 - t113313;
    t119149
}
