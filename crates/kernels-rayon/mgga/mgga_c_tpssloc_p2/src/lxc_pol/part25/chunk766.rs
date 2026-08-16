//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 766/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk766(t2244: f64, t751: f64, t2658: f64, t9853: f64, t9859: f64, t9911: f64, t9914: f64, t9917: f64, t9921: f64, t9923: f64, t9925: f64, t9928: f64, t9931: f64) -> (f64, f64) {
    let t9932 = t751 * t2244;
    let t9933 = t2658 * t9932;
    let t9934 = 36.0_f64 * t9933;
    let t9935 = t9853 + t9911 + t9914 + t9917 - t9921 - t9923 + t9925 + t9859 + t9928 + t9931 + t9934;
    (t9934, t9935)
}
