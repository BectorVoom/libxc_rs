//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1040/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1040(t11397: f64, t277: f64, t332: f64, t7877: f64, t959: f64, t11399: f64, t2547: f64, t3788: f64, t3784: f64, t11311: f64, t2619: f64, t1086: f64, t6182: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11977 = t277 * t11397;
    let t11979 = t7877 * t959 * t332;
    let t11980 = t11399 * t11979;
    let t11981 = t11977 * t11980;
    let t11983 = t2547 * t3788;
    let t11984 = t3784 * t11983;
    let t11986 = t2619 * t11311;
    let t11987 = t1086 * t6182;
    (t11977, t11979, t11980, t11981, t11983, t11984, t11986, t11987)
}
