//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1020/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1020(t12701: f64, t572: f64, t12751: f64, t5137: f64, t639: f64, t10848: f64, t2643: f64, t12638: f64, t1630: f64, t12642: f64, t17440: f64, t1006: f64, t10485: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41514 = t12701 * t572;
    let t41524 = t639 * t5137 * t12751;
    let t41562 = t10848 * t2643;
    let t41570 = t639 * t1630 * t12638;
    let t41573 = t639 * t17440 * t12642;
    let t41595 = t1006 * t10485;
    (t41514, t41524, t41562, t41570, t41573, t41595)
}
