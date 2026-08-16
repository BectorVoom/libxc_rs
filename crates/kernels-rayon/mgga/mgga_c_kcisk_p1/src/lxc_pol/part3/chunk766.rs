//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 766/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk766(t11774: f64, t11228: f64, t719: f64, t735: f64, t10522: f64, t641: f64, t746: f64, t741: f64, t5310: f64, t5327: f64, t10431: f64, t5322: f64, sigma2: f64) -> (f64, f64, f64, f64) {
    let t11775 = t11774 * sigma2;
    let t11776 = t719 * t11228;
    let t11777 = t735 * t11776;
    let t11778 = t11775 * t11777;
    let t11780 = t641 * t10522;
    let t11781 = t746 * t11780;
    let t11782 = t741 * t11781;
    let t11784 = t5310 * t5327;
    let t11786 = t5322 * t10431;
    (t11778, t11782, t11784, t11786)
}
