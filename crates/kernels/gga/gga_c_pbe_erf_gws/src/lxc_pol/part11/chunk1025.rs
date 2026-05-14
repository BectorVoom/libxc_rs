//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1025/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1025<F: Float>(t42530: F, t18838: F, t18850: F, t18853: F, t18863: F, t18920: F, t18924: F, t18933: F, t18939: F, t48495: F, t48496: F, t48497: F, t48498: F, t33581: F, t33583: F, t22653: F) -> (F, F, F, F, F) {
    let t48499 = 4.0 * t42530;
    let t48500 = -t48495 - t48496 - t18838 + t18850 + t18920 + t18924 + t18853 - t48497 - t48498 + t48499 - t18863 - t18933 + t18939;
    let t48502 = 72.0 * t33581;
    let t48503 = 192.0 * t33583;
    let t48504 = 0.23392893589820816284e1 * t22653;
    (t48499, t48500, t48502, t48503, t48504)
}
