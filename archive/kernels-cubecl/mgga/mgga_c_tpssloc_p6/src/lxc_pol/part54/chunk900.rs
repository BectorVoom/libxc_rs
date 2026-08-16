//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 900/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk900<F: Float>(t225: F, t4149: F, t4947: F, t4943: F, t4941: F, t5053: F, t3701: F, t5356: F, t5213: F, t5211: F, t1372: F, t1824: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13463 = t4149 * t225;
    let t14972 = t4947 * t225;
    let t14980 = t4943 * t225;
    let t15797 = t4941 * t225;
    let t15820 = t5053 * t225;
    let t15868 = t5356 * t3701;
    let t16022 = t5213 * t225;
    let t16030 = t5211 * t225;
    let t16036 = t1372 * t1824;
    (t13463, t14972, t14980, t15797, t15820, t15868, t16022, t16030, t16036)
}
