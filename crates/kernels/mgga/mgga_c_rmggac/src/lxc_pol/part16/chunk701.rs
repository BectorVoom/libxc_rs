//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 701/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk701<F: Float>(t31: F, t35604: F, t7349: F, t7351: F, t7338: F, t7345: F, t7341: F, t1338: F, t2039: F, t357: F, t638: F, t132: F, t4781: F, t1343: F, t2040: F, t71: F, t830: F) -> (F, F, F, F, F, F) {
    let t35728 = t7349 * t7351 * t35604 * t31;
    let t35742 = t7345 * t7338;
    let t35744 = t7345 * t7341;
    let t35772 = t638 * t2039 * t357 * t1338;
    let t35776 = t638 * t2039 * t132 * t4781;
    let t35781 = t638 * t830 * t1343 * t71 * t2040;
    (t35728, t35742, t35744, t35772, t35776, t35781)
}
