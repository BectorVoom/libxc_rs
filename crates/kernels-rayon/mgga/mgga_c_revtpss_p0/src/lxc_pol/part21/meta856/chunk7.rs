//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3254/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3254(t5: f64, t10301: f64, t10309: f64, t10310: f64, t10313: f64, t13272: f64, t13286: f64, t13289: f64, t13420: f64, t2247: f64, t2248: f64, t2315: f64, t4178: f64, t4241: f64, t45931: f64, t45933: f64, t45941: f64, t45944: f64, t45952: f64, t45958: f64, t60214: f64, t60215: f64, t60216: f64, t60217: f64, t60218: f64, t60221: f64, t60224: f64, t60496: f64, t644: f64, t91: f64) -> f64 {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t60498 = piecewise3(t8, 0.0_f64, (-t60214 + t45931 - t45933 + t60215 - t60216 + t45941 - t45944 + t60217 - t60218 + t45952) * t91 + 60.0_f64 * t60221 * t2248 - 120.0_f64 * t60224 * t10310 + 60.0_f64 * t13272 * t10313 + 60.0_f64 * t45958 * t4178 + 120.0_f64 * t10301 * t13286 + 60.0_f64 * t10301 * t13289 - 360.0_f64 * t10309 * t4241 * t2248 + 60.0_f64 * t2247 * t13420 * t644 + 60.0_f64 * t2247 * t4241 * t2315 + t60496);
    t60498
}
