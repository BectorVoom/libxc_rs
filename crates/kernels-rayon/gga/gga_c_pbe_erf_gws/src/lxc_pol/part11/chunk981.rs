//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 981/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk981(t1333: f64, t3361: f64, t10020: f64, t1392: f64, t1336: f64, t1438: f64, t1218: f64, t10016: f64, t414: f64, t1448: f64, t3360: f64, t4: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t33550 = t1333 * t3361;
    let t33572 = t10020 * t1392;
    let t33581 = t1336 * t3361;
    let t33583 = t1438 * t3361;
    let t33596 = t10020 * t1218;
    let t33598 = t414 * t10016;
    let t33604 = t3360 * t4 * t1448;
    (t33550, t33572, t33581, t33583, t33596, t33598, t33604)
}
