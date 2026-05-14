//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 931/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk931<F: Float>(t13069: F, t19: F, t796: F, t801: F, t13156: F, t817: F, t13536: F, t2142: F, t3854: F, t8884: F, t3065: F, t858: F, t11667: F, t3916: F, t13580: F, t13524: F, t8978: F, t9246: F) -> (F, F, F, F, F, F, F, F) {
    let t44395 = t13069 * t796 * t19 * t801;
    let t44405 = t13156 * t817;
    let t44465 = t13536 * t2142;
    let t44477 = t8884 * t3854;
    let t44479 = t3065 * t858 * t44477;
    let t44530 = t3916 * t11667;
    let t44537 = t13580 * t2142;
    let t44577 = t8978 * t9246 * t13524;
    (t44395, t44405, t44465, t44477, t44479, t44530, t44537, t44577)
}
