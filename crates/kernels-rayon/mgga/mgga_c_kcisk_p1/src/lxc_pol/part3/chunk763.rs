//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 763/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk763(t11749: f64, t735: f64, t734: f64, t10534: f64, t5322: f64, t5321: f64, t1954: f64, t5307: f64, t1931: f64, t5303: f64, t1945: f64, t5336: f64) -> (f64, f64, f64, f64, f64) {
    let t11750 = t735 * t11749;
    let t11751 = t734 * t11750;
    let t11753 = t5322 * t10534;
    let t11754 = t5321 * t11753;
    let t11756 = t5307 * t1954;
    let t11758 = t1931 * t5303;
    let t11760 = t1945 * t5336;
    (t11751, t11754, t11756, t11758, t11760)
}
