//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1129/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1129(t2376: f64, t4182: f64, t829: f64, t830: f64, t1162: f64, t875: f64, t13796: f64, t3989: f64, t1105: f64, t4052: f64, t2409: f64, t4164: f64, t8734: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14435 = t2376 * t4182;
    let t14437 = t829 * t830 * t14435;
    let t14442 = t1162 * t875;
    let t14443 = t13796 * t14442;
    let t14444 = t3989 * t14443;
    let t14446 = t4052 * t1105;
    let t14448 = t2409 * t2376 * t14446;
    let t14452 = t2409 * t8734 * t4164;
    (t14437, t14443, t14444, t14446, t14448, t14452)
}
