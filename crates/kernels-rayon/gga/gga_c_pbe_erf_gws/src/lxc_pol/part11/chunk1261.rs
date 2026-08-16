//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1261/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1261(t44257: f64, t9035: f64, t2300: f64, t2343: f64, t2345: f64, t3814: f64, t44710: f64, t45863: f64, t45887: f64, t49178: f64, t49986: f64, t50019: f64, t50027: f64, t50036: f64, t50041: f64, t50043: f64, t904: f64, t914: f64, t916: f64, t929: f64) -> (f64, f64) {
    let t50045 = t9035 * t44257 / 4.0_f64;
    let t50046 = t49986 - 7.0_f64 / 288.0_f64 * t45863 - t914 * t916 * t904 * t50019 / 1536.0_f64 - t50027 + t2343 * t2345 * t44710 * t3814 / 96.0_f64 + 5.0_f64 / 192.0_f64 * t929 * t2300 * t904 * t49178 + t50036 - 7.0_f64 / 48.0_f64 * t45887 + t50041 - t50043 + t50045;
    (t50045, t50046)
}
