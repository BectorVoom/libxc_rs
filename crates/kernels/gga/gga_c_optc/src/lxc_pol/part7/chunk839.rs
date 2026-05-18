//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 839/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk839<F: Float>(t7995: F, t8003: F, t7845: F, t953: F, t7848: F, t2367: F, t2629: F, t930: F, t2668: F, t2797: F, t2803: F, t2806: F, t2812: F, t3884: F, t3907: F, t7852: F, t7859: F, t7992: F, t7996: F, t7999: F) -> (F, F, F) {
    let t8004 = t7995 * t8003;
    let t8007 = t953 * t7845;
    let t8009 = t953 * t7848;
    let t8019 = t2367 * t2629;
    let t8020 = t930 * t8019;
    let t8022 = -F::new(0.4395493670620718481e3) * t3884 * t7992 - F::new(0.15486228121497046737e2) * t2668 * t7996 + F::new(0.1169609647897054359e2) * t2812 * t7999 + F::new(0.4645868436449114021e2) * t3907 * t8004 + F::new(0.10076140891672839458e-1) * t8007 + F::new(0.16793568152788065762e-1) * t8009 + F::new(0.50380704458364197288e-2) * t953 * t7852 + F::new(0.22391424203717421017e-1) * t953 * t7859 - F::new(0.23181763972770020946e0) * t2797 * t2803 - F::new(0.30909018630360027928e0) * t2797 * t2806 + F::new(0.28977204965962526182e-1) * t8020;
    (t8004, t8019, t8022)
}
