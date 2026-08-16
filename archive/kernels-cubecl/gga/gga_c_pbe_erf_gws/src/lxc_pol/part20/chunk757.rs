//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 757/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk757<F: Float>(t1321: F, t83: F, t4607: F, t1224: F, t1229: F, t1232: F, t1254: F, t1262: F, t1300: F, t1305: F, t1315: F, t1320: F, t1323: F, t155: F, t174: F, t435: F, t442: F, t457: F, t4664: F, t4674: F, t4679: F, t4682: F, t4688: F, t4689: F, t4697: F, t4701: F, t4711: F, t4714: F, t4718: F, t4719: F, t4723: F, t4730: F, t4735: F) -> (F, F) {
    let t4737 = F::cast_from(1.0_f64) / t1321 / t83;
    let t4738 = t4607 * t4737;
    let t4741 = -t4664 + F::cast_from(1.0_f64) * t435 * t4674 + F::cast_from(0.20691336878655965246e4_f64) * t4679 * t4682 - t4688 + F::cast_from(0.32530742648344572643e-1_f64) * t174 * t4689 * t1305 + F::cast_from(0.10274e0_f64) * t174 * t155 * t1229 * t1232 - F::cast_from(0.48159446095139119799e0_f64) * t174 * t4697 * t1323 + F::cast_from(0.21687161765563048428e-1_f64) * t174 * t4701 * t457 - F::cast_from(0.16265371324172286321e-1_f64) * t174 * t1300 * t1315 - t4711 + t4714 + t4718 - F::cast_from(0.16522997748472177549e1_f64) * t174 * t4719 * t1262 + F::cast_from(0.68493333333333333332e-1_f64) * t174 * t4723 * t442 - F::cast_from(0.51369999999999999999e-1_f64) * t174 * t1224 * t1254 + F::cast_from(0.35089340384731224426e1_f64) * t1320 * t4730 + F::cast_from(0.1025389702100779493e4_f64) * t4735 * t4738;
    (t4737, t4741)
}
