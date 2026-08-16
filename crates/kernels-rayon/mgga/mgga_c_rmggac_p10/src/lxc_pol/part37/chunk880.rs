//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 880/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk880(t1322: f64, t75770: f64, t880: f64, t899: f64, t1326: f64, t75311: f64, t68815: f64, t15105: f64, t352: f64, t68729: f64, t14011: f64, t14052: f64, t8615: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t75771 = t899 * t880 * t1322 * t75770;
    let t75773 = t1326 * t75311;
    let t75774 = t68815 * t75773;
    let t75779 = t1326 * t15105 * t352;
    let t75780 = t68729 * t75779;
    let t75789 = t14052 * t14011 * t8615;
    (t75771, t75773, t75774, t75779, t75780, t75789)
}
