//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1262/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1262(t7884: f64, t9858: f64, t1230: f64, t125: f64, t19557: f64, t19568: f64, t19571: f64, t19574: f64, t19577: f64, t19579: f64, t19664: f64, t22991: f64, t22994: f64, t22997: f64, t27007: f64, t27015: f64, t27018: f64, t27021: f64, t27023: f64, t27025: f64, t2986: f64, t555: f64, t557: f64) -> f64 {
    let t27027 = t7884 * t9858;
    let t27034 = -5.0_f64 / 432.0_f64 * t22991 + t22994 / 72.0_f64 + t22997 / 72.0_f64 + t27007 / 288.0_f64 + t19557 + t19568 / 48.0_f64 - 5.0_f64 / 144.0_f64 * t19571 - 5.0_f64 / 144.0_f64 * t19574 + t19577 / 96.0_f64 - 5.0_f64 / 144.0_f64 * t19579 + t19664 / 48.0_f64 - t27015 / 32.0_f64 - t27018 / 32.0_f64 - t27021 / 32.0_f64 - t27023 / 16.0_f64 - t27025 / 16.0_f64 + 7.0_f64 / 16.0_f64 * t27027 - t555 * t2986 * t557 * t1230 * t125 / 16.0_f64;
    t27034
}
