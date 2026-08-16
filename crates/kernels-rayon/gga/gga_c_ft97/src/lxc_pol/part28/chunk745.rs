//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 745/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk745(t1693: f64, t395: f64, t44: f64, t5551: f64, t1302: f64, t1291: f64, t1295: f64, t2035: f64, t22623: f64, t22687: f64, t32181: f64, t32185: f64, t32187: f64, t32190: f64, t32208: f64, t401: f64, t5518: f64, t5530: f64, t5534: f64, t5557: f64, t5604: f64, t7178: f64, t7181: f64, t7318: f64, t7867: f64, t79: f64) -> (f64, f64, f64, f64) {
    let t32211 = t1693 * t395;
    let t32213 = 1.0_f64 / t44 / t32211;
    let t32214 = t5551 * t32213;
    let t32215 = t32214 * t1302;
    let t32218 = t32181 + 0.20429954681481481482e0_f64 * t7178 * t5604 - t32185 + 0.11854761295685025975e-1_f64 * t79 * t32187 - 0.19762785756235085044e-4_f64 * t7867 * t2035 * t32190 - 0.88910709717637694816e-2_f64 * t5518 * t1291 - 0.88910709717637694816e-2_f64 * t5534 * t1291 - 0.21080304806650757379e-3_f64 * t1295 * t5557 + 0.47419045182740103902e-1_f64 * t1295 * t5530 + 0.39525571512470170088e-4_f64 * t22687 * t2035 * t7318 * t401 + 0.52700762016626893448e-4_f64 * t7181 * t32208 + 0.78129887353338233165e-6_f64 * t22623 * t32215;
    (t32213, t32214, t32215, t32218)
}
