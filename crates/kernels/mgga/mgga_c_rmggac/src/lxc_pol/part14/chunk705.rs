//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 705/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk705<F: Float>(t35889: F, t7633: F, t2103: F, t35864: F, t2115: F, t35876: F, t2118: F, t35925: F, t35872: F, t2100: F, t25518: F, t27: F, t25640: F, t25636: F, t2084: F, t798: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t36074 = t7633 * t35889;
    let t36078 = t2103 * t35864;
    let t36088 = t2115 * t35876;
    let t36090 = t2118 * t35925;
    let t36092 = t2115 * t35872;
    let t36094 = t2100 * t35876;
    let t36096 = t2103 * t35925;
    let t36099 = t2118 * t35864;
    let t36101 = t2100 * t35872;
    let t36103 = t25518 * t27;
    let t36107 = t25640 * t27;
    let t36110 = t25636 * t27;
    let t36114 = t2084 * t798;
    (t36074, t36078, t36088, t36090, t36092, t36094, t36096, t36099, t36101, t36103, t36107, t36110, t36114)
}
