//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 412/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk412(t14: f64, t1906: f64, t1647: f64, t653: f64, t621: f64, t632: f64, t645: f64, t190: f64, t650: f64, t1743: f64, t225: f64, t664: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1907 = t14 * t1906;
    let t1910 = 0.96491876992155210402e2_f64 * t1907 * t653 * t1647;
    let t1913 = 4.0_f64 * t632 * t645 * t621;
    let t1916 = 6.0_f64 * t650 * t190 * t1647;
    let t1917 = t1743 * t225;
    let t1923 = t664 * t664;
    (t1907, t1910, t1913, t1916, t1917, t1923)
}
