//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 657/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk657<F: Float>(t309: F, t799: F, t4152: F, t8392: F, t1882: F, t4173: F, t4188: F, t4178: F, t4183: F, t4267: F, t1526: F, t4406: F, t7705: F, t339: F, t39: F, t11: F, t340: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t15460 = t799 * t309;
    let t15467 = 2.0 / 27.0 * t8392 * t4152;
    let t15471 = 2.0 / 27.0 * t1882 * t4173;
    let t15491 = 2.0 / 27.0 * t1882 * t4188;
    let t15500 = 2.0 / 9.0 * t1882 * t4178;
    let t15502 = 4.0 / 9.0 * t1882 * t4183;
    let t15532 = 4.0 / 27.0 * t8392 * t4267;
    let t15562 = t1526 * t7705 * t4406;
    let t15564 = t339 * t39;
    let t15565 = t340 * t11;
    (t15460, t15467, t15471, t15491, t15500, t15502, t15532, t15562, t15564, t15565)
}
