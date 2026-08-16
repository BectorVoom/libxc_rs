//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 986/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk986(t3380: f64, t700: f64, t145: f64, t3379: f64, t242: f64, t10201: f64, t41: f64, t168: f64, t3609: f64, t703: f64, t10283: f64, t11151: f64, t245: f64, t7981: f64, t8042: f64, t8047: f64, t8050: f64, t8051: f64, t8057: f64, t8058: f64) -> (f64, f64, f64) {
    let t11157 = t3380 * t700;
    let t11159 = t145 * t3379;
    let t11160 = t11159 * t242;
    let t11162 = t41 * t10201;
    let t11166 = t168 * t703 * t3609;
    let t11168 = 0.26574420457892358282e1_f64 * t7981 - 0.3350512821420176075e0_f64 * t8042 - 0.56945186695483624892e0_f64 * t10283 - t8047 - 0.11938374665504764976e-1_f64 * t168 * t245 * t11151 + t8050 + 0.3350512821420176075e0_f64 * t8051 - t8057 - 0.16752564107100880375e0_f64 * t8058 - 0.83762820535504401876e-1_f64 * t11157 + 0.83762820535504401876e-1_f64 * t11160 - 0.83762820535504401876e-1_f64 * t11162 * t242 + 0.19897291109174608293e-1_f64 * t11166;
    (t11159, t11162, t11168)
}
