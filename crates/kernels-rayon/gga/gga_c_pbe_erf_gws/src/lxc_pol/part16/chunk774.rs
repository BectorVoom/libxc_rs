//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 774/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk774(t153: f64, t274: f64, t4573: f64, t1592: f64, t475: f64, t142: f64, t1504: f64, t525: f64, t1354: f64, t285: f64, t545: f64, t281: f64) -> (f64, f64, f64, f64) {
    let t5595 = 0.4429070076315393047e1_f64 * t153 * t4573 * t274;
    let t5598 = t475 * t1592;
    let t5602 = t142 * t1504;
    let t5603 = t525 * t5602;
    let t5607 = t1354 * t545 * t285;
    let t5608 = t281 * t5607;
    (t5595, t5598, t5603, t5608)
}
