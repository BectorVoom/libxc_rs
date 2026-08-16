//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1143/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1143(t23384: f64, t30789: f64, t30908: f64, t225: f64, t30844: f64, t30808: f64, t1945: f64, t6733: f64, t30783: f64, t82431: f64, t344: f64, t6688: f64, t6768: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t113207 = t23384 * t30789;
    let t113217 = t23384 * t30908;
    let t113219 = t30844 * t225;
    let t113231 = t30808 * t225;
    let t113236 = t6733 * t1945;
    let t113240 = t82431 * t30783;
    let t113243 = t344 * t1945 * t225;
    let t113261 = t6688 * t6768;
    (t113207, t113217, t113219, t113231, t113236, t113240, t113243, t113261)
}
