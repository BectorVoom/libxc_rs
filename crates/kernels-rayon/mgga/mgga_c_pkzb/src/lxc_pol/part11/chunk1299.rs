//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1299/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1299(t237: f64, t31151: f64, t31191: f64, t31309: f64, t31345: f64, t31391: f64, t31437: f64, t31517: f64, t31587: f64, t1217: f64, t27501: f64, t11353: f64, t2328: f64) -> (f64, f64, f64) {
    let t31591 = t237 * (t31151 + t31191 + t31309 + t31345 + t31391 + t31437 + t31517 + t31587);
    let t31593 = 0.17544670867903938621e1_f64 * t27501 * t1217;
    let t31595 = 0.5848223622634646207e0_f64 * t2328 * t11353;
    (t31591, t31593, t31595)
}
