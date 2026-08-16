//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1298/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1298(t112: f64, t30581: f64, t111808: f64, t1268: f64, t1849: f64, t19451: f64, t19456: f64, t2200: f64, t2202: f64, t2314: f64, t26117: f64, t28007: f64, t30266: f64, t30315: f64, t30538: f64, t30543: f64, t30565: f64, t4028: f64, t4034: f64, t5107: f64, t5113: f64, t574: f64, t652: f64, t75560: f64, t7676: f64, t8176: f64, t8190: f64, t8194: f64, t8196: f64, t8273: f64, t8280: f64, t96657: f64, t96709: f64) -> (f64, f64) {
    let t111845 = t30581 * t112;
    let t111916 = 2.0_f64 * t111808 * t1268 * t574 + 4.0_f64 * t1268 * t1849 * t30315 - 4.0_f64 * t5107 * t652 * t8273 - 2.0_f64 * t19451 * t8176 - 2.0_f64 * t19451 * t8190 + 4.0_f64 * t19456 * t8280 - 2.0_f64 * t2200 * t75560 + 2.0_f64 * t2202 * t75560 + 2.0_f64 * t2202 * t96657 + 2.0_f64 * t2202 * t96709 + 4.0_f64 * t2314 * t30538 - 4.0_f64 * t2314 * t30543 + 2.0_f64 * t2314 * t30565 + 4.0_f64 * t26117 * t8280 + 2.0_f64 * t28007 * t8194 + 2.0_f64 * t28007 * t8196 + 4.0_f64 * t30266 * t4028 + 4.0_f64 * t30266 * t7676 + 4.0_f64 * t30538 * t5113 - 4.0_f64 * t30543 * t4034;
    (t111845, t111916)
}
