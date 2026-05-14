//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1031/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1031<F: Float>(t2271: F, t6670: F, t822: F, t6674: F, t20206: F, t2407: F, t858: F, t6672: F, t6335: F, t6342: F, t6800: F, t6605: F, t6702: F, t6183: F, t6706: F, t2120: F) -> (F, F, F, F, F) {
    let t20743 = t2271 * t6670;
    let t20744 = t822 * t20743;
    let t20746 = t20744 * t6674 / 4.0;
    let t20748 = t2407 * t858 * t20206;
    let t20750 = t6672 * t20748 / 4.0;
    let t20753 = t6800 * t6335 * t6342 / 16.0;
    let t20754 = t6702 * t6605;
    let t20755 = 7.0 / 36.0 * t20754;
    let t20756 = t6183 * t6706;
    let t20757 = t2120 * t20756;
    (t20746, t20750, t20753, t20755, t20757)
}
