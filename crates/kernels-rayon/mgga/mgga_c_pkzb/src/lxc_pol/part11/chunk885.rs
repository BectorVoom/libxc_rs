//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 885/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk885(t2888: f64, t9554: f64, t2106: f64, t3685: f64, t2105: f64, t2029: f64, t3650: f64, t2901: f64, t302: f64, t2923: f64, t2976: f64, t3645: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9555 = t2888 * t9554;
    let t9558 = t3685 * t2106;
    let t9559 = t2105 * t9558;
    let t9562 = t3650 * t2029;
    let t9563 = t9562 * t2901;
    let t9564 = t302 * t9563;
    let t9567 = t9562 * t2923;
    let t9568 = t302 * t9567;
    let t9571 = t2976 * t3645;
    (t9555, t9558, t9559, t9562, t9563, t9564, t9567, t9568, t9571)
}
