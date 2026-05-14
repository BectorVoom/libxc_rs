//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 721/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk721<F: Float>(t36920: F, t7933: F, t7935: F, t7490: F, t7932: F, t7936: F, t2185: F, t678: F, t7943: F, t7344: F, t14267: F, t71: F, t132: F, t270: F, t31: F, t35688: F) -> (F, F, F, F, F, F, F, F) {
    let t36922 = t7933 * t36920 * t7935;
    let t36924 = t7490 * t7932;
    let t36925 = t36924 * t7936;
    let t36928 = t7943 * t2185 * t678;
    let t36935 = t7344 * t7932;
    let t36936 = t36935 * t7936;
    let t36938 = t14267 * t71;
    let t36940 = t132 * t270 * t31;
    let t36942 = t35688 * t36938 * t36940;
    (t36922, t36924, t36925, t36928, t36935, t36936, t36940, t36942)
}
