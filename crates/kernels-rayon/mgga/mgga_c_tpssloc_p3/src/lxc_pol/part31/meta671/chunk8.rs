//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2009/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2009(t111: f64, t28942: f64, t5456: f64, t7039: f64, t100990: f64, t102310: f64, t1268: f64, t12725: f64, t1458: f64, t19451: f64, t19456: f64, t2039: f64, t27170: f64, t28002: f64, t4028: f64, t671: f64, t7056: f64, t75560: f64, t7801: f64, t92090: f64, t96356: f64, t96683: f64, t96709: f64) -> (f64, f64, f64) {
    let t102386 = t28942 * t111;
    let t102401 = t7039 * t5456;
    let t102403 = 2.0_f64 * t100990 * t1268 + 2.0_f64 * t102386 * t671 + 4.0_f64 * t12725 * t7801 + 4.0_f64 * t1458 * t92090 + 2.0_f64 * t19451 * t7056 + 4.0_f64 * t19456 * t7801 + 2.0_f64 * t2039 * t75560 + 4.0_f64 * t2039 * t96356 + 4.0_f64 * t2039 * t96683 + 2.0_f64 * t2039 * t96709 + 4.0_f64 * t27170 * t4028 + 4.0_f64 * t28002 * t7056 + t102310 + 2.0_f64 * t102401;
    (t102386, t102401, t102403)
}
