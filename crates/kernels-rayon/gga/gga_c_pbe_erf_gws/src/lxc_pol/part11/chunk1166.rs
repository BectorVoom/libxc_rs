//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1166/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1166(t33550: f64, t18610: f64, t18619: f64, t18624: f64, t18626: f64, t18629: f64, t18645: f64, t18655: f64, t18658: f64, t18667: f64, t18709: f64, t18914: f64) -> (f64, f64) {
    let t48493 = 120.0_f64 * t33550;
    let t48494 = -t18610 - t18619 - t18624 + t48493 - t18626 - t18629 - t18645 + t18655 + t18658 - t18667 + t18709 + t18914;
    (t48493, t48494)
}
