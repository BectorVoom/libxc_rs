//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 806/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk806<F: Float>(t12323: F, t247: F, t251: F, t10607: F, t10611: F, t10633: F, t12592: F, t12593: F, t12595: F, t12598: F, t12601: F, t12602: F, t12603: F, t12604: F, t12605: F, t12607: F, t12608: F, t12611: F, t12615: F, t12619: F, t256: F) -> (F, F, F) {
    let t13008 = t12323 * t247;
    let t13009 = t13008 * t251;
    let t13013 = -t12592 + t12593 + t13009 * t256 / F::cast_from(3.0_f64) + t12595 - t12598 - t12601 + t12602 + t12603 - t12604 - t12605 + t10607 + F::cast_from(0.18233333333333333333e0_f64) * t10611 + t12607 - t12608 + t10633 - t12611 + t12615 - t12619;
    (t13008, t13009, t13013)
}
