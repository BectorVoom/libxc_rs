//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 531/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk531<F: Float>(t1882: F, t5724: F, t1314: F, t8232: F, t376: F, t5706: F, t89: F, t5637: F, t5646: F, t5719: F, t8392: F, t103: F, t5617: F, t1332: F, t1851: F, t5733: F) -> (F, F, F, F, F, F, F, F, F) {
    let t23176 = t1882 * t5724;
    let t23183 = 4.0 / 27.0 * t8232 * t1314;
    let t23199 = t89 * t376 * t5706;
    let t23227 = t1882 * t5637;
    let t23229 = t1882 * t5646;
    let t23239 = t8392 * t5719;
    let t23244 = t103 * t5617;
    let t23249 = t1851 * t1332;
    let t23263 = t1882 * t5733;
    (t23176, t23183, t23199, t23227, t23229, t23239, t23244, t23249, t23263)
}
