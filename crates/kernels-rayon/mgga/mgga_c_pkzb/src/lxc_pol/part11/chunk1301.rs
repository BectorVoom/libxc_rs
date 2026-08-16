//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1301/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1301(t3147: f64, t9756: f64, t11180: f64, t18509: f64, t18513: f64, t889: f64, t898: f64, t11213: f64, t2317: f64, t3161: f64, t10151: f64, t10159: f64) -> (f64, f64, f64, f64, f64) {
    let t31599 = 0.70178683471615754484e1_f64 * t3147 * t9756;
    let t31604 = 0.91082604192152556044e5_f64 * t898 * t18509 * t11180 * t18513 * t889;
    let t31605 = t2317 * t11213;
    let t31608 = 0.17315859105681463759e2_f64 * t898 * t31605 * t3161;
    let t31610 = 0.31168546390226634765e3_f64 * t3147 * t10151;
    let t31612 = 0.17544670867903938621e1_f64 * t3147 * t10159;
    (t31599, t31604, t31608, t31610, t31612)
}
