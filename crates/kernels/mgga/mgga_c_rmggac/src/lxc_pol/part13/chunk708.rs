//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 708/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk708<F: Float>(t34805: F, t648: F, t35765: F, t793: F, t305: F, t35590: F, t35885: F, t7653: F, t7641: F, t35889: F, t7648: F, t7633: F, t2103: F, t35864: F, t2115: F, t35876: F) -> (F, F, F, F, F, F, F, F, F) {
    let t36034 = t648 * t34805;
    let t36045 = t793 * t35765;
    let t36058 = t305 * t35590;
    let t36063 = t7653 * t35885;
    let t36065 = t7641 * t35885;
    let t36072 = t7648 * t35889;
    let t36074 = t7633 * t35889;
    let t36078 = t2103 * t35864;
    let t36088 = t2115 * t35876;
    (t36034, t36045, t36058, t36063, t36065, t36072, t36074, t36078, t36088)
}
