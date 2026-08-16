//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1043/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1043(t1058: f64, t637: f64, t452: f64, t987: f64, t1147: f64, t803: f64, t568: f64, t2507: f64, t42: f64, t1259: f64, t955: f64, t14: f64, t4494: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12223 = t1058 * t637;
    let t12227 = t987 * t452;
    let t12389 = t1147 * t803;
    let t12419 = t1058 * t568;
    let t12431 = t2507 * t42;
    let t12919 = t1259 * t955;
    let t13925 = t14 * t4494;
    (t12223, t12227, t12389, t12419, t12431, t12919, t13925)
}
