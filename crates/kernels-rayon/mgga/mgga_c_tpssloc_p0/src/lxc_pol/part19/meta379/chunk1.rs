//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1417/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1417(t43936: f64, t43949: f64, t449: f64, t300: f64, t1098: f64, t11470: f64, t1119: f64, t11180: f64, t3308: f64, t3256: f64, t3312: f64, t3316: f64) -> (f64, f64, f64, f64, f64) {
    let t43951 = (t43936 + t43949) * t449;
    let t43953 = 0.19751673498613801407e-1_f64 * t300 * t43951;
    let t43954 = t11470 * t1098;
    let t43956 = 4.0_f64 * t43954 * t1119;
    let t43958 = 6.0_f64 * t11180 * t3308;
    let t43959 = t3256 * t3312;
    let t43961 = 0.96491876992155210402e2_f64 * t43959 * t3316;
    (t43951, t43953, t43956, t43958, t43961)
}
