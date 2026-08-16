//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2238/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2238(t1041: f64, t13969: f64, t17975: f64, t17687: f64, t14085: f64, t4571: f64, t13765: f64, t13995: f64, t18086: f64, t3069: f64, t10952: f64, t17655: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t61919 = t1041 * t13969 * t17975;
    let t61923 = t1041 * t13969 * t17687;
    let t61929 = t14085 * t4571;
    let t61940 = t13995 * t13765;
    let t61950 = t18086 * t3069;
    let t61975 = t10952 * t17655;
    (t61919, t61923, t61929, t61940, t61950, t61975)
}
