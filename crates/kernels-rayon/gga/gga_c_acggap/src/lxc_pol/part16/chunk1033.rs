//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1033/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1033(t36351: f64, t31773: f64, t8916: f64, t7447: f64, t8920: f64, t1439: f64, t1983: f64, t7380: f64, t1460: f64, t1992: f64, t2095: f64, t30225: f64, t532: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t36352 = 0.12862205435420921092e-2_f64 * t36351;
    let t36353 = t31773 * t8916;
    let t36354 = 0.3361875e0_f64 * t36353;
    let t36355 = t7447 * t8920;
    let t36356 = 0.16809375e0_f64 * t36355;
    let t36364 = t7380 * t1983 * t1439;
    let t36365 = t36364 / 32.0_f64;
    let t36367 = t2095 * t1992 * t1460;
    let t36368 = t36367 / 48.0_f64;
    let t36370 = t30225 * t532;
    (t36352, t36354, t36356, t36365, t36368, t36370)
}
