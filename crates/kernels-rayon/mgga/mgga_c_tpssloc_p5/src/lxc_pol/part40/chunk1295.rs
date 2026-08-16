//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1295/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1295(t1266: f64, t1268: f64, t1774: f64, t19456: f64, t2181: f64, t2183: f64, t2314: f64, t28002: f64, t30180: f64, t30181: f64, t30189: f64, t30211: f64, t30424: f64, t30425: f64, t30428: f64, t30444: f64, t30447: f64, t30454: f64, t4028: f64, t4034: f64, t5107: f64, t5113: f64, t5361: f64, t6468: f64, t652: f64, t8124: f64, t8143: f64, t8230: f64, t8235: f64, t96356: f64, t96683: f64) -> f64 {
    let t111503 = -2.0_f64 * t1266 * t30424 * t652 + 4.0_f64 * t1268 * t5361 * t8230 + 2.0_f64 * t1268 * t6468 * t8143 - 4.0_f64 * t1774 * t30180 * t652 - 4.0_f64 * t5107 * t652 * t8230 + 4.0_f64 * t19456 * t8235 - 4.0_f64 * t2181 * t96356 + 4.0_f64 * t2183 * t96683 + 2.0_f64 * t2314 * t30425 + 4.0_f64 * t2314 * t30428 - 4.0_f64 * t2314 * t30444 - 2.0_f64 * t2314 * t30447 - 4.0_f64 * t28002 * t8124 + 4.0_f64 * t30181 * t4028 - 4.0_f64 * t30189 * t4028 + 4.0_f64 * t30211 * t4028 + 2.0_f64 * t30425 * t5113 - 4.0_f64 * t30444 * t4034 - 2.0_f64 * t30447 * t4034 + 2.0_f64 * t30454 * t5113;
    t111503
}
