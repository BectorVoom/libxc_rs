//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 930/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk930<F: Float>(t1121: F, t6491: F, t3438: F, t5175: F, t15068: F, t5091: F, t1195: F, t6731: F, t382: F, t19789: F, t5176: F, t1166: F, t6705: F) -> (F, F, F, F, F, F, F, F, F) {
    let t19856 = t6491 * t1121;
    let t19857 = t3438 * t19856;
    let t19858 = t5175 * t19857;
    let t19860 = t15068 * t5091;
    let t19862 = t1195 * t6731;
    let t19863 = t382 * t19862;
    let t19865 = t5176 * t19789;
    let t19866 = t5175 * t19865;
    let t19868 = t1166 * t6705;
    (t19856, t19857, t19858, t19860, t19862, t19863, t19865, t19866, t19868)
}
