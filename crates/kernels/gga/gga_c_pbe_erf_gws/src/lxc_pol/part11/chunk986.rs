//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 986/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk986<F: Float>(t47833: F, t5218: F, t7149: F, t7049: F, t12599: F, t24835: F, t30170: F, t3406: F, t5211: F, t3390: F, t3469: F, t4927: F, t639: F, t1033: F, t12585: F, t32093: F) -> (F, F, F, F, F, F, F) {
    let t47836 = 64.0 / 15.0 * t5218 * t7149 * t47833;
    let t47839 = 32.0 / 9.0 * t5218 * t7049 * t47833;
    let t47841 = 64.0 / 15.0 * t24835 * t12599;
    let t47844 = 32.0 / 15.0 * t5211 * t30170 * t3406;
    let t47848 = 32.0 / 15.0 * t639 * t4927 * t3469 * t3390;
    let t47850 = 16.0 / 5.0 * t1033 * t12585;
    let t47851 = 16.0 / 45.0 * t32093;
    (t47836, t47839, t47841, t47844, t47848, t47850, t47851)
}
