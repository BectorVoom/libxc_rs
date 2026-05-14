//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 546/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk546<F: Float>(t305: F, t7817: F, t648: F, t7561: F, t2068: F, t7638: F, t2067: F, t3839: F) -> (F, F, F, F) {
    let t7818 = t305 * t7817;
    let t7819 = 0.14635184302277988245e0 * t7818;
    let t7820 = t648 * t7561;
    let t7821 = 0.33335697577410973224e-1 * t7820;
    let t7826 = t2068 * t7638;
    let t7829 = t3839 * t2067;
    (t7819, t7821, t7826, t7829)
}
