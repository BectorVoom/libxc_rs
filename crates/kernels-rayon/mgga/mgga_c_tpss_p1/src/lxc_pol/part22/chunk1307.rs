//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1307/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1307(t1288: f64, t2433: f64, t19817: f64, t44474: f64, t2133: f64, t10897: f64, t30: f64, t2116: f64, t44350: f64, t2436: f64, t10514: f64, t1398: f64, t2428: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t63794 = t1288 * t2433;
    let t63797 = t19817 * t44474;
    let t63806 = t1288 * t2133;
    let t63817 = t30 * t10897;
    let t63823 = t1288 * t2116;
    let t63837 = t19817 * t44350;
    let t63840 = t2436 * t1288;
    let t63841 = t63840 * t10514;
    let t63844 = t1398 * t2428;
    (t63794, t63797, t63806, t63817, t63823, t63837, t63841, t63844)
}
