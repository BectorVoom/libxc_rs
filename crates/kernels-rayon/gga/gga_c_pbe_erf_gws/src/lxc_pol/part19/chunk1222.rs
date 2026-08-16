//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1222/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1222(t1211: f64, t21885: f64, t804: f64, t20091: f64, t4090: f64, t2416: f64, t4110: f64, t22509: f64, t4099: f64, t4083: f64, t4424: f64, t51869: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t52105 = t1211 * t21885;
    let t52112 = t804 * t1211;
    let t52159 = t20091 * t4090;
    let t52191 = t2416 * t4110;
    let t52251 = t22509 * t4099;
    let t52353 = t4424 * t4083;
    let t52525 = 595.0_f64 / 5184.0_f64 * t51869;
    (t52105, t52112, t52159, t52191, t52251, t52353, t52525)
}
