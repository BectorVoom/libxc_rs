//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1259/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1259<F: Float>(t49463: F, t823: F, t850: F, t852: F, t860: F, t3703: F, t1130: F, t11706: F, t13086: F, t13325: F, t13328: F, t13331: F, t21010: F, t2181: F, t27917: F, t3154: F, t339: F, t340: F, t3717: F, t38451: F, t3848: F, t3851: F, t45901: F, t48985: F, t49436: F, t49955: F, t6429: F, t870: F, t9056: F) -> (F, F, F) {
    let t49986 = t850 * t49463 * t823 * t852 * t860 / F::cast_from(96.0_f64);
    let t50002 = t3703 * t3703;
    let t50018 = -F::cast_from(48.0_f64) * t1130 * t13086 * t2181 - F::cast_from(360.0_f64) * t21010 * t339 * t50002 - F::cast_from(36.0_f64) * t2181 * t339 * t49955 - t339 * t340 * t49436 + F::cast_from(3.0_f64) * t339 * t48985 * t870 + F::cast_from(360.0_f64) * t3717 * t3848 * t6429 + F::cast_from(12.0_f64) * t1130 * t45901 + F::cast_from(18.0_f64) * t11706 * t3851 + F::cast_from(240.0_f64) * t13325 * t27917 - F::cast_from(144.0_f64) * t13328 * t9056 + F::cast_from(12.0_f64) * t13331 * t3154 - F::cast_from(72.0_f64) * t38451 * t3848;
    (t49986, t50002, t50018)
}
