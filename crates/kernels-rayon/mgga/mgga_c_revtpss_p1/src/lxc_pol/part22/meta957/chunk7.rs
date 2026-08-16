//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3214/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3214(t10298: f64, t10301: f64, t10309: f64, t13283: f64, t13420: f64, t1497: f64, t21674: f64, t21677: f64, t21682: f64, t2247: f64, t2248: f64, t2315: f64, t4178: f64, t4241: f64, t45926: f64, t45931: f64, t45933: f64, t45936: f64, t45941: f64, t45944: f64, t45947: f64, t45952: f64, t45958: f64, t45963: f64, t45972: f64, t5816: f64, t5872: f64, t60224: f64, t603: f64, t60793: f64, t60829: f64, t60871: f64, t60994: f64, t91: f64) -> f64 {
    let t61007 = 40.0_f64 * t2247 * t1497 * t13420 + 40.0_f64 * t10301 * t21682 + 20.0_f64 * t45958 * t5816 - 480.0_f64 * t10309 * t4178 * t4241 - 4.0_f64 * t10298 * t5872 + 20.0_f64 * t2247 * t5872 * t2315 - 240.0_f64 * t60224 * t13283 + 840.0_f64 * t45972 * t5816 * t2248 - 240.0_f64 * t45963 * t21674 + 80.0_f64 * t10301 * t21677 - 4.0_f64 * t603 * (t60793 + t60829 + t60871 + t60994) - 120.0_f64 * t10309 * t5816 * t2315 + (-24.0_f64 * t45926 + t45931 + t45933 - 480.0_f64 * t45936 + t45941 + t45944 - 2520.0_f64 * t45947 + t45952) * t91;
    t61007
}
