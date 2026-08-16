//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1211/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1211(t14322: f64, t4414: f64, t1206: f64, t2182: f64, t353: f64, t8599: f64, t14291: f64, t9270: f64, t22509: f64, t4099: f64, t14266: f64, t14311: f64, t2367: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t52217 = t4414 * t14322;
    let t52241 = t8599 * t353 * t1206 * t2182;
    let t52249 = t9270 * t14291;
    let t52251 = t22509 * t4099;
    let t52263 = t9270 * t14266;
    let t52266 = t2367 * t14311;
    (t52217, t52241, t52249, t52251, t52263, t52266)
}
