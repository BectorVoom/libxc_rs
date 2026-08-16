//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1017/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1017(t119: f64, t1477: f64, t465: f64, t84: f64, t1273: f64, t1276: f64, t174: f64, t331: f64, t4540: f64, t4715: f64, t15652: f64, t36: f64, t88: f64) -> (f64, f64, f64, f64) {
    let t18467 = 0.18989760778855128827e-2_f64 * t465 * t119 * t1477 * t84;
    let t18471 = 0.28493333333333333334e0_f64 * t174 * t331 * t1273 * t1276;
    let t18474 = 0.4274e0_f64 * t174 * t4715 * t4540;
    let t18477 = 840.0_f64 * t36 * t15652 * t88;
    (t18467, t18471, t18474, t18477)
}
