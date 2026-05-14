//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1193/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1193<F: Float>(t6029: F, t97681: F, t491: F, t7381: F, t7953: F, t21971: F, t4261: F, t7952: F, t5748: F, t6034: F, t1468: F, t22452: F, t22376: F, t22674: F, t28624: F, t22373: F, t27520: F) -> (F, F, F, F, F, F, F, F) {
    let t102916 = t97681 * t6029;
    let t102918 = t7381 * t491;
    let t102919 = t102918 * t7953;
    let t102922 = t7952 * t4261 * t21971;
    let t102924 = t5748 * t6034;
    let t102926 = t1468 * t22452;
    let t102928 = t7952 * t22376;
    let t102930 = t28624 * t22674;
    let t102932 = t27520 * t22373;
    (t102916, t102919, t102922, t102924, t102926, t102928, t102930, t102932)
}
