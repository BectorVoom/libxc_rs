//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 745/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk745<F: Float>(t1291: F, t2039: F, t270: F, t638: F, t2046: F, t2050: F, t31: F, t1277: F, t2085: F, t7315: F, t5016: F, t7707: F) -> (F, F, F, F, F, F) {
    let t35114 = t638 * t2039 * t1291 * t270;
    let t35118 = t2046 * t2050 * t1291 * t31;
    let t35124 = t638 * t2039 * t1277 * t270;
    let t35128 = t2046 * t2050 * t1277 * t31;
    let t35130 = t7315 * t2085;
    let t35132 = t5016 * t7707;
    (t35114, t35118, t35124, t35128, t35130, t35132)
}
