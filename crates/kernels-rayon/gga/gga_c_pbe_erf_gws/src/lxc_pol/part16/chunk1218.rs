//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1218/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1218(t51414: f64, t51458: f64, t4116: f64, t6854: f64, t14369: f64, t321: f64, t14166: f64, t2429: f64, t1167: f64, t2423: f64, t3324: f64, t810: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t52696 = 595.0_f64 / 2592.0_f64 * t51414;
    let t52715 = 455.0_f64 / 648.0_f64 * t51458;
    let t52751 = t4116 * t6854;
    let t52755 = t321 * t14369;
    let t52757 = t2429 * t14166;
    let t52763 = t1167 * t2423;
    let t52767 = t3324 * t810;
    (t52696, t52715, t52751, t52755, t52757, t52763, t52767)
}
