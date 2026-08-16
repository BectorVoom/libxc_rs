//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1121/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1121(t23340: f64, t1229: f64, t17955: f64, t918: f64, t1238: f64, t6428: f64, t19191: f64, t2380: f64, t3224: f64, t6382: f64, t3201: f64, t5939: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t23341 = 0.7622047665434619906e-3_f64 * t23340;
    let t23345 = t918 * t17955 * t1229;
    let t23355 = t1238 * t6428;
    let t23366 = t2380 * t19191 * t3224;
    let t23367 = 0.28582678745379824648e-3_f64 * t23366;
    let t23381 = t1238 * t6382;
    let t23382 = 0.15244095330869239812e-2_f64 * t23381;
    let t23388 = t918 * t5939 * t3201;
    (t23341, t23345, t23355, t23367, t23382, t23388)
}
