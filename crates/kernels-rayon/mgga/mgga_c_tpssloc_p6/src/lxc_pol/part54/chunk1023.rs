//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1023/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1023(t26193: f64, t6907: f64, t1985: f64, t225: f64, t5318: f64, t567: f64, t214: f64, t1377: f64, t1842: f64, t1307: f64, t22635: f64, t22633: f64) -> (f64, f64, f64, f64) {
    let t26206 = t26193 * t6907;
    let t26207 = t1985 * t26206;
    let t26210 = t5318 * t225 * t567;
    let t26211 = t214 * t26210;
    let t26212 = t1985 * t26211;
    let t26214 = t1377 * t1842;
    let t26215 = t26214 * t1307;
    let t26216 = t22635 * t26215;
    let t26217 = t22633 * t26216;
    (t26207, t26212, t26215, t26217)
}
