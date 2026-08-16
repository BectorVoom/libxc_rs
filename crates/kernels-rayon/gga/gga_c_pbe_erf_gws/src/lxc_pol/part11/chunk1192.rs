//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1192/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1192(t11275: f64, t12399: f64, t12412: f64, t12413: f64, t125: f64, t18106: f64, t18108: f64, t18122: f64, t18126: f64, t26012: f64, t26085: f64, t26101: f64, t26118: f64, t26129: f64, t26135: f64, t2857: f64, t33691: f64, t33770: f64, t33778: f64, t33854: f64, t34302: f64, t3671: f64, t3686: f64, t42304: f64, t48609: f64, t48706: f64, t5651: f64, t8497: f64, t967: f64, t981: f64) -> f64 {
    let t48709 = -0.3486808982146430324e-2_f64 * t33691 + t18106 - t18108 + 0.65586876954174354395e-3_f64 * t26012 - 24.0_f64 * t8497 * t34302 * t12412 - 0.18276876377896586758e-4_f64 * t26085 - 0.47896936041018436376e-1_f64 * t26101 - 24.0_f64 * t33854 * t12413 - 3.0_f64 * t3686 * t12399 - 0.31931290694012290916e0_f64 * t33770 + t18122 + t18126 + 6.0_f64 * t11275 * t5651 * t981 * t3671 + 0.11890099055206112556e1_f64 * t33778 + 0.78054266140918933351e0_f64 * t26118 - 0.63862581388024581833e0_f64 * t26129 - 0.21862292318058118132e-3_f64 * t26135 + 24.0_f64 * t2857 * t42304 * t967 + (t48609 + t48706) * t125;
    t48709
}
