//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1169/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1169<F: Float>(t6310: F, t6627: F, t6484: F, t6530: F, t20296: F, t2168: F, t2170: F, t2171: F, t20264: F, t20832: F, t20837: F, t20840: F, t20846: F, t20848: F, t20849: F, t20855: F, t3140: F, t3235: F, t3247: F) -> (F, F, F) {
    let t20856 = t6627 * t6310;
    let t20858 = t6484 * t6530;
    let t20859 = F::new(7.0) / F::new(12.0) * t20858;
    let t20863 = t2168 * t2170 * t20296 * t2171 / F::new(12.0);
    let t20868 = t20832 - t20837 + F::new(119.0) / F::new(144.0) * t20840 + t20846 + t20848 - F::new(7.0) / F::new(48.0) * t20849 - t20855 + F::new(7.0) / F::new(192.0) * t20856 - t20859 + t20863 + t3247 * t3235 * t20264 * t3140 / F::new(128.0);
    (t20859, t20863, t20868)
}
