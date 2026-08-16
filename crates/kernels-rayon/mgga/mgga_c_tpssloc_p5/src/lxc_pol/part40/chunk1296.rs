//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1296/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1296(t1268: f64, t12725: f64, t1393: f64, t19451: f64, t2181: f64, t2183: f64, t26114: f64, t26117: f64, t28002: f64, t28007: f64, t30186: f64, t30189: f64, t30203: f64, t30211: f64, t30424: f64, t30428: f64, t4028: f64, t5113: f64, t55943: f64, t7458: f64, t75560: f64, t7676: f64, t8144: f64, t8148: f64, t8150: f64, t8231: f64, t8235: f64, t96657: f64, t96709: f64) -> f64 {
    let t111546 = 2.0_f64 * t1268 * t1393 * t30424 - 4.0_f64 * t12725 * t8231 - 2.0_f64 * t19451 * t8144 + 2.0_f64 * t19451 * t8148 + 2.0_f64 * t19451 * t8150 - 2.0_f64 * t2181 * t55943 + 2.0_f64 * t2183 * t75560 + 2.0_f64 * t2183 * t96657 + 2.0_f64 * t2183 * t96709 - 4.0_f64 * t26114 * t8231 + 4.0_f64 * t26114 * t8235 + 4.0_f64 * t26117 * t8235 + 4.0_f64 * t28002 * t8150 + 2.0_f64 * t28007 * t8148 + 4.0_f64 * t30186 * t4028 + 4.0_f64 * t30186 * t7676 - 4.0_f64 * t30189 * t7458 - 4.0_f64 * t30203 * t7458 + 4.0_f64 * t30211 * t7676 + 4.0_f64 * t30428 * t5113;
    t111546
}
