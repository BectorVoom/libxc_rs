//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 929/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk929<F: Float>(t43: F, t50: F, t18668: F, t260: F, t1402: F, t1403: F, t1407: F, t16669: F, t16679: F, t16746: F, t4360: F, t47: F, t4757: F, t4760: F, t262: F, t1412: F, t1413: F, t1416: F, t16973: F, t16978: F, t16986: F, t4373: F, t4767: F, t4770: F, t52: F, zeta_threshold: F) -> (F, F, F) {
    let t44 = t43 <= zeta_threshold;
    let t51 = t50 <= zeta_threshold;
    let t18669 = 0.23392893589820816284e1 * t18668;
    let t18670 = 1.0 / t260;
    let t18683 = piecewise3(t44, 0.0, 40.0 / 81.0 * t18670 * t16669 - 16.0 / 9.0 * t4757 * t1403 * t1407 + 4.0 / 3.0 * t1402 * t16679 + 16.0 / 9.0 * t4760 * t4360 + 4.0 / 3.0 * t47 * t16746);
    let t18684 = 1.0 / t262;
    let t18697 = piecewise3(t51, 0.0, 40.0 / 81.0 * t18684 * t16973 - 16.0 / 9.0 * t4767 * t1413 * t1416 + 4.0 / 3.0 * t1412 * t16986 + 16.0 / 9.0 * t4770 * t4373 + 4.0 / 3.0 * t52 * t16978);
    (t18669, t18683, t18697)
}
