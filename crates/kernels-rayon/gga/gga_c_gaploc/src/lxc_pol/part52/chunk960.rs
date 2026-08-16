//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 960/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk960(t426: f64, t49921: f64, t12116: f64, t14272: f64, t14277: f64, t2268: f64, t2756: f64, t3701: f64, t44530: f64, t44534: f64, t44538: f64, t44542: f64, t44544: f64, t44549: f64, t44552: f64, t44553: f64, t44556: f64, t44559: f64, t44572: f64, t44574: f64, t44576: f64, t535: f64, t6305: f64, t6313: f64, t988: f64) -> f64 {
    let t49944 = t49921 * t426;
    let t49958 = 0.56910013271352299198e-1_f64 * t2268 * t535 * t49944 - t44530 + t44534 - t44538 + t44542 - t44544 - t44549 + t44552 + t44553 + 0.15176003539027279787e0_f64 * t6313 * t14272 + 0.56910013271352299198e-1_f64 * t6305 * t14277 + 0.56910013271352299198e-1_f64 * t2268 * t12116 * t988 + 0.56910013271352299198e-1_f64 * t2268 * t3701 * t2756 + t44556 + t44559 + t44572 - t44574 + t44576;
    t49958
}
