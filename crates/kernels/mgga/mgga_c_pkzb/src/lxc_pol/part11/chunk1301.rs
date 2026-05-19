//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1301/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1301<F: Float>(t3147: F, t9756: F, t11180: F, t18509: F, t18513: F, t889: F, t898: F, t11213: F, t2317: F, t3161: F, t10151: F, t10159: F) -> (F, F, F, F, F) {
    let t31599 = F::cast_from(0.70178683471615754484e1_f64) * t3147 * t9756;
    let t31604 = F::cast_from(0.91082604192152556044e5_f64) * t898 * t18509 * t11180 * t18513 * t889;
    let t31605 = t2317 * t11213;
    let t31608 = F::cast_from(0.17315859105681463759e2_f64) * t898 * t31605 * t3161;
    let t31610 = F::cast_from(0.31168546390226634765e3_f64) * t3147 * t10151;
    let t31612 = F::cast_from(0.17544670867903938621e1_f64) * t3147 * t10159;
    (t31599, t31604, t31608, t31610, t31612)
}
