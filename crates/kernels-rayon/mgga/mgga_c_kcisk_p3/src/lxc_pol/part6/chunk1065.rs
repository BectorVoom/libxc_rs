//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 1065/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk1065(t2326: f64, t7706: f64, t14978: f64, t1312: f64, t14909: f64, t3952: f64, t30852: f64, t4391: f64, t7710: f64, t4400: f64, t2059: f64, t8398: f64) -> (f64, f64, f64, f64, f64) {
    let t31464 = t7706 * t2326;
    let t31465 = t14978 * t31464;
    let t31466 = t1312 * t31465;
    let t31469 = t14909 * t31464;
    let t31470 = t3952 * t31469;
    let t31473 = t4391 * t30852;
    let t31474 = t3952 * t31473;
    let t31477 = t7710 * t2326;
    let t31478 = t4400 * t31477;
    let t31479 = t1312 * t31478;
    let t31483 = t4400 * t2059 * t8398;
    (t31466, t31470, t31474, t31479, t31483)
}
