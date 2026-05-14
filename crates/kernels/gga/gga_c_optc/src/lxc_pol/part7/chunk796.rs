//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 796/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk796<F: Float>(t8019: F, t930: F, t2668: F, t2797: F, t2803: F, t2806: F, t2812: F, t3884: F, t3907: F, t7852: F, t7859: F, t7992: F, t7996: F, t7999: F, t8004: F, t8007: F, t8009: F, t953: F) -> (F,) {
    let t8020 = t930 * t8019;
    let t8022 = -0.4395493670620718481e3 * t3884 * t7992 - 0.15486228121497046737e2 * t2668 * t7996 + 0.1169609647897054359e2 * t2812 * t7999 + 0.4645868436449114021e2 * t3907 * t8004 + 0.10076140891672839458e-1 * t8007 + 0.16793568152788065762e-1 * t8009 + 0.50380704458364197288e-2 * t953 * t7852 + 0.22391424203717421017e-1 * t953 * t7859 - 0.23181763972770020946e0 * t2797 * t2803 - 0.30909018630360027928e0 * t2797 * t2806 + 0.28977204965962526182e-1 * t8020;
    (t8022,)
}
