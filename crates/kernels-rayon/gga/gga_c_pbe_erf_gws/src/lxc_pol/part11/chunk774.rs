//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 774/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk774(t10617: f64, t12582: f64, t12587: f64, t12592: f64, t12593: f64, t12595: f64, t12598: f64, t12601: f64, t12602: f64, t12603: f64, t12604: f64, t12605: f64, t12607: f64, t5906: f64, t5919: f64, t5922: f64, t8425: f64) -> (f64, f64) {
    let t12608 = 8.0_f64 / 15.0_f64 * t10617;
    let t12609 = -t12582 - t12587 + t5906 - t12592 + t12593 + 0.33545228223331014468e-1_f64 * t8425 - t5919 + t5922 + t12595 - t12598 - t12601 + t12602 + t12603 - t12604 - t12605 + t12607 - t12608;
    (t12608, t12609)
}
