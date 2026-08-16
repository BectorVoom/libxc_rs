//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1295/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1295(t18885: f64, t18939: f64, t18941: f64, t18950: f64, t18954: f64, t18956: f64, t18959: f64, t19537: f64, t2429: f64, t321: f64, t3703: f64, t3931: f64, t3932: f64, t39758: f64, t48502: f64, t48503: f64, t48504: f64, t48506: f64, t48507: f64, t48508: f64) -> f64 {
    let t50767 = -36.0_f64 * t2429 * t3703 * t3932 + 12.0_f64 * t321 * t3931 * t39758 + t18885 + t18939 + t18941 - t18950 + t18954 + t18956 + t18959 - t19537 + t48502 + t48503 - t48504 + t48506 + t48507 - t48508;
    t50767
}
