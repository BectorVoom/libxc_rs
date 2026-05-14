//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1175/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1175<F: Float>(t3147: F, t9756: F, t11180: F, t18509: F, t18513: F, t889: F, t898: F, t11213: F, t2317: F, t3161: F, t10151: F, t10159: F, t1306: F, t31186: F, t31188: F, t31190: F, t31196: F, t31198: F, t3282: F, t9759: F) -> (F, F, F, F, F, F) {
    let t31599 = 0.70178683471615754484e1 * t3147 * t9756;
    let t31604 = 0.91082604192152556044e5 * t898 * t18509 * t11180 * t18513 * t889;
    let t31605 = t2317 * t11213;
    let t31608 = 0.17315859105681463759e2 * t898 * t31605 * t3161;
    let t31610 = 0.31168546390226634765e3 * t3147 * t10151;
    let t31612 = 0.17544670867903938621e1 * t3147 * t10159;
    let t31616 = -3.0 * t1306 * t3282 * t9759 + t31186 + t31188 + t31190 - t31196 + t31198 + t31599 - t31604 - t31608 + t31610 - t31612;
    (t31599, t31604, t31608, t31610, t31612, t31616)
}
