//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1519/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1519(t25: f64, t1788: f64, t2225: f64, t2221: f64, t2223: f64, t12130: f64, t11987: f64, t1408: f64, t2: f64, t3704: f64, t1298: f64, t15941: f64, t16: f64, t2249: f64, t3665: f64, t5170: f64, t5173: f64, t584: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t15982 = t2225 * t1788;
    let t15983 = 20.0_f64 * t15982;
    let t15984 = t2221 * t1788;
    let t15985 = 12.0_f64 * t15984;
    let t15986 = t2223 * t1788;
    let t15987 = 32.0_f64 * t15986;
    let t15988 = 2.0_f64 * t12130;
    let t15989 = t11987 * t1408;
    let t15992 = t3704 * t2;
    let t16002 = piecewise3(t26, 0.0_f64, 8.0_f64 / 27.0_f64 * t15989 * t3665 - 8.0_f64 / 9.0_f64 * t15992 * t15941 - 2.0_f64 / 9.0_f64 * t5170 * t2249 + 4.0_f64 / 3.0_f64 * t1298 * t584 - 4.0_f64 * t5173 * t16);
    (t15983, t15985, t15987, t15988, t16002)
}
