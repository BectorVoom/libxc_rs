//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1152/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1152(t3553: f64, t1792: f64, t186: f64, t211: f64, t16985: f64, t40962: f64, t47942: f64, t47946: f64, t47951: f64, t47955: f64, t47959: f64, t47994: f64, t47997: f64, t48000: f64, t48003: f64) -> (f64, f64) {
    let t48326 = t3553 * t3553;
    let t48330 = 4.0_f64 / 5.0_f64 * t211 * t186 * t1792 * t48326;
    let t48341 = -0.5037777777777777778e-2_f64 * t40962 + 0.45340000000000000001e-1_f64 * t47942 - 0.45340000000000000002e-1_f64 * t47994 + 0.37783333333333333335e-2_f64 * t47946 + 0.5037777777777777778e-2_f64 * t47997 - 0.4534e-1_f64 * t47951 + 0.6801e-1_f64 * t48000 - 0.11335e-1_f64 * t47955 - 0.15113333333333333333e-1_f64 * t48003 - t16985 + 0.55975308641975308645e-2_f64 * t47959;
    (t48330, t48341)
}
