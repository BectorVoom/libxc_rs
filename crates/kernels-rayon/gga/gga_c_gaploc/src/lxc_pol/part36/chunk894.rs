//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 894/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk894(t27232: f64, t3145: f64, t8045: f64, t9256: f64, t12856: f64, t17293: f64, t605: f64, t1382: f64, t2497: f64, t3418: f64, t32100: f64, t921: f64) -> (f64, f64, f64, f64, f64) {
    let t42503 = 2.0_f64 * t27232 * t3145;
    let t42506 = 4.0_f64 * t8045 * t9256;
    let t42509 = 24.0_f64 * t17293 * t12856 * t605;
    let t42511 = t1382 * t3418 * t2497;
    let t42512 = 4.0_f64 * t42511;
    let t42513 = t32100 * t921;
    (t42503, t42506, t42509, t42512, t42513)
}
