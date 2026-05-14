//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1196/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1196<F: Float>(t5002: F, t963: F, t1422: F, t2452: F, t2321: F, t2461: F, t6880: F, t955: F, t1527: F, t7741: F, t2788: F, t4962: F, t1509: F, t2483: F, t41: F, t4878: F, t898: F) -> (F, F, F, F, F, F, F, F) {
    let t23795 = t963 * t5002;
    let t23797 = t1422 * t2452;
    let t23798 = 96.0 * t23797;
    let t23799 = t2321 * t2461;
    let t23800 = 3.0 * t23799;
    let t23820 = t6880 * t955;
    let t23828 = t7741 * t1527;
    let t23829 = 0.32530743900905219526e-1 * t23828;
    let t23830 = t2788 * t4962;
    let t23834 = t41 * t2483 * t1509;
    let t23835 = 3.0 * t23834;
    let t23837 = t41 * t898 * t4878;
    (t23795, t23798, t23800, t23820, t23829, t23830, t23835, t23837)
}
