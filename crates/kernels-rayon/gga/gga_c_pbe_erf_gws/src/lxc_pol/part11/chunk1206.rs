//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1206/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1206(t1105: f64, t13087: f64, t13720: f64, t18577: f64, t18580: f64, t18587: f64, t18594: f64, t18599: f64, t18604: f64, t18607: f64, t18610: f64, t18619: f64, t18624: f64, t18626: f64, t18629: f64, t18645: f64, t2429: f64, t48489: f64, t48493: f64, t804: f64) -> f64 {
    let t48957 = 24.0_f64 * t1105 * t13087 * t2429 + 24.0_f64 * t1105 * t13720 * t804 + t18577 + t18580 + t18587 + t18594 + t18599 - t18604 - t18607 - t18610 - t18619 - t18624 - t18626 - t18629 - t18645 - t48489 + t48493;
    t48957
}
