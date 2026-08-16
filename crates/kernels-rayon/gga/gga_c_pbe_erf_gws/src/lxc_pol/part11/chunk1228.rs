//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1228/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1228(t18885: f64, t18950: f64, t18954: f64, t18956: f64, t18959: f64, t18961: f64, t18968: f64, t19537: f64, t48502: f64, t48503: f64, t48504: f64, t48506: f64, t48507: f64, t48508: f64) -> f64 {
    let t49429 = -t19537 + t48502 + t48503 - t18950 + t18954 - t48504 + t48506 + t48507 + t18956 + t18885 - t48508 + t18959 - t18961 - t18968;
    t49429
}
