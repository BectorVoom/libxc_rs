//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 610/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk610<F: Float>(t4607: F, t4734: F, t4737: F, t470: F, t449: F, t456: F, t4619: F, t1327: F, t414: F, t1319: F, t455: F, t4623: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t4782 = t4734 * t4607 * t4737;
    let t4783 = t470 * t4782;
    let t4784 = F::cast_from(0.1025389702100779493e4_f64) * t4783;
    let t4788 = t449 * t4619 * t456;
    let t4789 = t470 * t4788;
    let t4790 = F::cast_from(0.58482233974552040708e0_f64) * t4789;
    let t4798 = t414 * t1327;
    let t4799 = F::cast_from(12.0_f64) * t4798;
    let t4800 = t1319 * t455;
    let t4801 = t4800 * t4623;
    (t4782, t4783, t4784, t4788, t4789, t4790, t4798, t4799, t4800, t4801)
}
