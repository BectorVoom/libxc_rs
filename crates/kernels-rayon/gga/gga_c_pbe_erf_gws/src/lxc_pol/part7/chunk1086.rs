//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1086/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1086(t18594: f64, t18596: f64, t18599: f64, t18601: f64, t18604: f64, t18607: f64, t18610: f64, t18612: f64, t18614: f64, t18619: f64, t18624: f64, t18626: f64, t18629: f64, t18631: f64, t18634: f64, t18636: f64, t4379: f64, t804: f64, t946: f64) -> f64 {
    let t19498 = 12.0_f64 * t4379 * t804 * t946 + t18594 + t18596 + t18599 - t18601 - t18604 - t18607 - t18610 + t18612 - t18614 - t18619 - t18624 + t18626 - t18629 - t18631 - t18634 - t18636;
    t19498
}
