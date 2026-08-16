//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 1035/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk1035(t30979: f64, t3482: f64, t30852: f64, t5625: f64, t3484: f64, t5634: f64, t5633: f64, t30184: f64, t3796: f64, t2173: f64, t26992: f64, t13293: f64, t30189: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t30980 = t3482 * t30979;
    let t30982 = t5625 * t30852;
    let t30983 = t3484 * t30982;
    let t30984 = t3482 * t30983;
    let t30986 = t5634 * t30852;
    let t30987 = t3484 * t30986;
    let t30988 = t5633 * t30987;
    let t30990 = t5634 * t30184;
    let t30991 = t3796 * t30990;
    let t30992 = t5633 * t30991;
    let t30994 = t26992 * t2173;
    let t31000 = t13293 * t30189;
    (t30980, t30984, t30988, t30992, t30994, t31000)
}
