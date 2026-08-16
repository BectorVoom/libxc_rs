//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1294/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1294(t109: f64, t111379: f64, t111413: f64, t1268: f64, t12725: f64, t19451: f64, t19456: f64, t2181: f64, t26114: f64, t26179: f64, t28002: f64, t28030: f64, t30195: f64, t30201: f64, t30203: f64, t30209: f64, t4028: f64, t574: f64, t7458: f64, t75560: f64, t8124: f64, t8144: f64, t8221: f64, t8231: f64, t8237: f64, t96683: f64, t96709: f64, t97933: f64) -> (f64, f64) {
    let t110 = 1.0_f64 < t109;
    let t111415 = piecewise3(t110, 0.0_f64, t111379 + t111413);
    let t111457 = 2.0_f64 * t111415 * t1268 * t574 - 4.0_f64 * t12725 * t8221 - 2.0_f64 * t19451 * t8124 - 4.0_f64 * t19456 * t8221 - 4.0_f64 * t19456 * t8231 + 4.0_f64 * t19456 * t8237 - 2.0_f64 * t2181 * t75560 - 4.0_f64 * t2181 * t96683 - 2.0_f64 * t2181 * t96709 - 2.0_f64 * t2181 * t97933 - 4.0_f64 * t26114 * t8221 - 4.0_f64 * t26179 * t8221 - 4.0_f64 * t28002 * t8144 - 2.0_f64 * t28030 * t8124 - 2.0_f64 * t28030 * t8144 - 4.0_f64 * t30195 * t4028 + 4.0_f64 * t30201 * t4028 - 4.0_f64 * t30203 * t4028 - 4.0_f64 * t30209 * t4028 - 4.0_f64 * t30209 * t7458;
    (t111415, t111457)
}
