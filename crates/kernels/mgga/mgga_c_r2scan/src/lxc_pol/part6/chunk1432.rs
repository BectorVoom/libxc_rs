//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1432/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1432<F: Float>(t6001: F, t7872: F, t23902: F, t23904: F, t23906: F, t23910: F, t23916: F, t23918: F, t23920: F, t26952: F, t26955: F, t26958: F, t765: F, t22595: F, t2823: F, t22602: F) -> (F, F, F) {
    let t26960 = t7872 * t6001;
    let t26961 = 0.2025780996e0 * t26960;
    let t26962 = -t23902 + 0.2025780996e0 * t765 * t26952 + 0.675260332e-1 * t765 * t26955 - t23904 - 0.2025780996e0 * t26958 + t23906 - t23910 - t23916 - t23918 + t23920 - t26961;
    let t26963 = t2823 * t22595;
    let t26964 = 0.2025780996e0 * t26963;
    let t26965 = t2823 * t22602;
    (t26962, t26964, t26965)
}
