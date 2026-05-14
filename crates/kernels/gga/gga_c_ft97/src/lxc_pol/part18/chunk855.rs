//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 855/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk855<F: Float>(t110: F, t1871: F, t22975: F, t1882: F, t5637: F, t5646: F, t1651: F, t5717: F, t1909: F, t1643: F, t3193: F, t5719: F, t8392: F, t5718: F, t8506: F, t103: F, t5617: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t23224 = t1871 * t110 * t22975;
    let t23227 = t1882 * t5637;
    let t23229 = t1882 * t5646;
    let t23231 = t5717 * t1651;
    let t23232 = t1909 * t23231;
    let t23235 = t5717 * t1643;
    let t23236 = t3193 * t23235;
    let t23239 = t8392 * t5719;
    let t23241 = t8506 * t5718;
    let t23244 = t103 * t5617;
    (t23224, t23227, t23229, t23231, t23232, t23235, t23236, t23239, t23241, t23244)
}
