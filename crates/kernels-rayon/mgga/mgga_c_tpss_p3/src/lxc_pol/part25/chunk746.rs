//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 746/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk746(t294: f64, t4946: f64, t4919: f64, t1457: f64, t3894: f64, t2593: f64, t4923: f64, t904: f64, t912: f64, t4939: f64, t895: f64, t2618: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4947 = t294 * t4946;
    let t4949 = 0.19751673498613801407e-1_f64 * t294 * t4919;
    let t4951 = 0.11696447245269292414e1_f64 * t3894 * t1457;
    let t4953 = t2593 * t4923 * t904;
    let t4955 = 0.11696447245269292414e1_f64 * t912 * t4953;
    let t4957 = t895 * t4939 * t904;
    let t4959 = 0.5848223622634646207e0_f64 * t912 * t4957;
    let t4960 = t2618 * t4923;
    (t4947, t4949, t4951, t4953, t4955, t4957, t4959, t4960)
}
