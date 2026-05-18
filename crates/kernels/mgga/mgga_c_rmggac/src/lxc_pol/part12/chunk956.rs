//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 956/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk956<F: Float>(t1971: F, t511: F, t615: F, t7230: F, t848: F, t34847: F, t8843: F, t1525: F, t352: F, t515: F, t866: F, t2320: F, t34878: F) -> (F, F, F, F, F) {
    let t40389 = t7230 * t1971 * t511 * t615 * t848;
    let t40391 = t34847 * t8843;
    let t40396 = t7230 * t1971 * t515 * t1525 * t352;
    let t40401 = t7230 * t1971 * t515 * t615 * t866;
    let t40403 = t34878 * t2320;
    (t40389, t40391, t40396, t40401, t40403)
}
