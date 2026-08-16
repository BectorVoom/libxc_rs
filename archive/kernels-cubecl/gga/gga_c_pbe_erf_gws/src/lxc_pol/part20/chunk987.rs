//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 987/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk987<F: Float>(t3380: F, t700: F, t145: F, t3379: F, t242: F, t10201: F, t41: F, t168: F, t3609: F, t703: F, t10283: F, t11151: F, t245: F, t7981: F, t8042: F, t8047: F, t8050: F, t8051: F, t8057: F, t8058: F) -> (F, F, F) {
    let t11157 = t3380 * t700;
    let t11159 = t145 * t3379;
    let t11160 = t11159 * t242;
    let t11162 = t41 * t10201;
    let t11166 = t168 * t703 * t3609;
    let t11168 = F::cast_from(0.26574420457892358282e1_f64) * t7981 - F::cast_from(0.3350512821420176075e0_f64) * t8042 - F::cast_from(0.56945186695483624892e0_f64) * t10283 - t8047 - F::cast_from(0.11938374665504764976e-1_f64) * t168 * t245 * t11151 + t8050 + F::cast_from(0.3350512821420176075e0_f64) * t8051 - t8057 - F::cast_from(0.16752564107100880375e0_f64) * t8058 - F::cast_from(0.83762820535504401876e-1_f64) * t11157 + F::cast_from(0.83762820535504401876e-1_f64) * t11160 - F::cast_from(0.83762820535504401876e-1_f64) * t11162 * t242 + F::cast_from(0.19897291109174608293e-1_f64) * t11166;
    (t11159, t11162, t11168)
}
