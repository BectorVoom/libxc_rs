//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 704/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk704<F: Float>(t36: F, t4616: F, t34805: F, t648: F, t305: F, t35590: F, t2115: F, t35876: F, t2118: F, t35925: F, t2100: F, t2103: F, t25518: F, t27: F, t25640: F, t25636: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t35972 = t4616 * t36;
    let t36034 = t648 * t34805;
    let t36058 = t305 * t35590;
    let t36088 = t2115 * t35876;
    let t36090 = t2118 * t35925;
    let t36094 = t2100 * t35876;
    let t36096 = t2103 * t35925;
    let t36103 = t25518 * t27;
    let t36107 = t25640 * t27;
    let t36110 = t25636 * t27;
    (t35972, t36034, t36058, t36088, t36090, t36094, t36096, t36103, t36107, t36110)
}
