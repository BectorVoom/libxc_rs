//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 839/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk839(t7995: f64, t8003: f64, t7845: f64, t953: f64, t7848: f64, t2367: f64, t2629: f64, t930: f64, t2668: f64, t2797: f64, t2803: f64, t2806: f64, t2812: f64, t3884: f64, t3907: f64, t7852: f64, t7859: f64, t7992: f64, t7996: f64, t7999: f64) -> (f64, f64, f64) {
    let t8004 = t7995 * t8003;
    let t8007 = t953 * t7845;
    let t8009 = t953 * t7848;
    let t8019 = t2367 * t2629;
    let t8020 = t930 * t8019;
    let t8022 = -0.4395493670620718481e3_f64 * t3884 * t7992 - 0.15486228121497046737e2_f64 * t2668 * t7996 + 0.1169609647897054359e2_f64 * t2812 * t7999 + 0.4645868436449114021e2_f64 * t3907 * t8004 + 0.10076140891672839458e-1_f64 * t8007 + 0.16793568152788065762e-1_f64 * t8009 + 0.50380704458364197288e-2_f64 * t953 * t7852 + 0.22391424203717421017e-1_f64 * t953 * t7859 - 0.23181763972770020946e0_f64 * t2797 * t2803 - 0.30909018630360027928e0_f64 * t2797 * t2806 + 0.28977204965962526182e-1_f64 * t8020;
    (t8004, t8019, t8022)
}
