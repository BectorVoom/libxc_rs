//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 494/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk494<F: Float>(t4239: F, t870: F, t1240: F, t2770: F, t848: F, t319: F, t871: F, t10478: F, t2766: F, t10491: F) -> (F, F, F, F, F, F, F, F) {
    let t15133 = t4239 * t870;
    let t15191 = t2770 * t1240;
    let t15195 = t848 * t1240;
    let t15229 = t2770 * t319;
    let t15254 = t848 * t871;
    let t15290 = t10478 * t319;
    let t15294 = t2766 * t871;
    let t15299 = t10491 * t319;
    (t15133, t15191, t15195, t15229, t15254, t15290, t15294, t15299)
}
