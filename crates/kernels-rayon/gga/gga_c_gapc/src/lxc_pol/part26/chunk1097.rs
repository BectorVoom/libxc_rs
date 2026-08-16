//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 1097/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk1097(t11923: f64, t11927: f64, t3363: f64, t1461: f64, t8710: f64, t1084: f64, t28517: f64, t26662: f64, t640: f64, t16798: f64, t7451: f64, t15548: f64, t7073: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t33617 = t3363 * t11923 * t11927;
    let t33619 = t1461 * t8710;
    let t33620 = t1084 * t33619;
    let t33621 = t33620 * t28517;
    let t33623 = t640 * t26662;
    let t33625 = t7451 * t33623 * t16798;
    let t33628 = t7073 * t33623 * t15548;
    (t33617, t33619, t33620, t33621, t33623, t33625, t33628)
}
