//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 972/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk972(t132: f64, t338: f64, t8949: f64, t3452: f64, t930: f64, t1386: f64, t2447: f64, t8589: f64, t1433: f64, t7108: f64, t2602: f64, t2579: f64, t3604: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t133 = t132 <= zeta_threshold;
    let t8950 = t8949 * t338;
    let t8951 = t3452 * t930;
    let t8953 = t1386 * t2447;
    let t8955 = piecewise3(t133, 0.0_f64, -t8589);
    let t8964 = t7108 * t1433;
    let t8965 = t8964 * t2602;
    let t8968 = t3604 * t2579;
    (t8950, t8951, t8953, t8955, t8965, t8968)
}
