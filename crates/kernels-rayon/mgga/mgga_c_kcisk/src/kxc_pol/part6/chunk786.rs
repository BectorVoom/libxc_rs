//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 786/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk786(t1248: f64, t13614: f64, t2075: f64, t2115: f64, t4030: f64, t4080: f64, t2201: f64, t3119: f64, t2206: f64, t3123: f64, t2198: f64, t3114: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20373 = t1248 * t13614 * t2075;
    let t20552 = t2115 * t4030;
    let t20567 = t2115 * t4080;
    let t20752 = t3119 * t2201;
    let t20754 = t3123 * t2206;
    let t20763 = t3114 * t2198;
    (t20373, t20552, t20567, t20752, t20754, t20763)
}
