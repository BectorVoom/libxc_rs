//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 983/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk983(t1368: f64, t281: f64, t285: f64, t3379: f64, t2030: f64, t3685: f64, t475: f64, t1251: f64, t1508: f64, t3649: f64, t10049: f64, t1243: f64) -> (f64, f64, f64, f64) {
    let t33849 = t281 * t3379 * t1368 * t285;
    let t33854 = t475 * t3685 * t2030;
    let t33963 = t1508 * t3649 * t1251;
    let t33965 = t10049 * t1243;
    (t33849, t33854, t33963, t33965)
}
