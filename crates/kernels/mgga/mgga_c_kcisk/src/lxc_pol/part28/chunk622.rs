//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 622/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk622<F: Float>(t1709: F, t7099: F, t4838: F, t4868: F, t7076: F, t7079: F, t7082: F, t7086: F, t1707: F, t2408: F, t4881: F, t1714: F, t1248: F, t2364: F, t4889: F, t4893: F, t6759: F) -> (F, F, F, F, F, F, F, F) {
    let t7100 = t7099 * t1709;
    let t7107 = t4868 + t4838 / 9.0 + t7076 / 9.0 - 2.0 / 9.0 * t7079 + 2.0 / 3.0 * t7082 + 2.0 / 3.0 * t7086;
    let t7108 = t1707 * t7107;
    let t7115 = t4881 * t2408;
    let t7116 = t7115 * t1709;
    let t7118 = t1714 * t7107;
    let t7122 = t1248 * t4889 * t2364;
    let t7125 = t1248 * t4893 * t6759;
    (t7100, t7107, t7108, t7115, t7116, t7118, t7122, t7125)
}
