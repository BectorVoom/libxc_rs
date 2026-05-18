//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 911/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk911<F: Float>(t3351: F, t515: F, t618: F, t7231: F, t866: F, t2283: F, t36542: F, t7404: F, t8571: F, t1635: F, t1971: F, t495: F, t7230: F, t880: F) -> (F, F, F, F) {
    let t39731 = t3351 * t7231 * t515 * t618 * t866;
    let t39733 = t36542 * t2283;
    let t39735 = t8571 * t7404;
    let t39742 = t7230 * t1971 * t880 * t1635 * t495;
    (t39731, t39733, t39735, t39742)
}
