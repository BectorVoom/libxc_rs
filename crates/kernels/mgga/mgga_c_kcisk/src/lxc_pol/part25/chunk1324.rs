//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1324/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1324<F: Float>(t2509: F, t415: F, t5204: F, t34115: F, t5074: F, t112637: F, t112645: F, t112648: F, t112661: F, t112663: F, t116584: F, t117093: F, t117097: F, t117106: F, t117110: F, t117113: F, t15930: F, t33031: F, t34016: F, t7242: F) -> (F, F, F) {
    let t117118 = t415 * t2509 * t5204;
    let t117120 = t5074 * t34115;
    let t117121 = 0.14739506172839506172e-2 * t117120;
    let t117124 = -0.1492375e-1 * t117093 + 0.55273148148148148147e-3 * t117097 - 0.77602083333333333335e-3 * t112637 - 0.69444444444444444446e-2 * t33031 * t116584 - 0.20833333333333333334e-1 * t33031 * t7242 * t34016 * t15930 + 0.89351851851851851854e-3 * t117106 - 0.44218518518518518517e-2 * t117110 + 0.3684876543209876543e-2 * t117113 + 0.23148148148148148148e-2 * t112645 + 0.89351851851851851853e-3 * t112648 - 0.55273148148148148147e-3 * t117118 + t117121 + 0.11054629629629629629e-2 * t112661 - 0.73697530864197530861e-3 * t112663;
    (t117118, t117120, t117124)
}
