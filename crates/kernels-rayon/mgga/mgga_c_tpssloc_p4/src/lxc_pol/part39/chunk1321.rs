//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1321/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1321(t109: f64, t111096: f64, t111141: f64, t1268: f64, t12734: f64, t19456: f64, t2200: f64, t2202: f64, t2314: f64, t26114: f64, t30035: f64, t30072: f64, t30088: f64, t30091: f64, t30266: f64, t30272: f64, t30316: f64, t30321: f64, t30326: f64, t4028: f64, t4034: f64, t5107: f64, t5361: f64, t55934: f64, t574: f64, t652: f64, t7676: f64, t8189: f64, t8190: f64, t8194: f64, t8196: f64, t8260: f64, t8280: f64, t90370: f64, t90381: f64, t9348: f64) -> (f64, f64) {
    let t110 = 1.0_f64 < t109;
    let t111143 = piecewise3(t110, 0.0_f64, t111096 + t111141);
    let t111168 = 4.0_f64 * t1268 * t8189 * t5361 - 4.0_f64 * t4034 * t30316 - 4.0_f64 * t2314 * t30272 - 4.0_f64 * t12734 * t8260 - 4.0_f64 * t2314 * t30326 + 2.0_f64 * t9348 * t8280 + 4.0_f64 * t4028 * t30091 - 2.0_f64 * t4028 * t30072 - 2.0_f64 * t90381 * t2200 + 2.0_f64 * t1268 * t111143 * t574 + 4.0_f64 * t90370 * t2202 + 4.0_f64 * t26114 * t8194 - 4.0_f64 * t55934 * t2200 + 4.0_f64 * t2314 * t30321 + 2.0_f64 * t7676 * t30035 + 2.0_f64 * t7676 * t30088 - 4.0_f64 * t652 * t5107 * t8189 + 4.0_f64 * t2314 * t30266 + 4.0_f64 * t19456 * t8196 - 4.0_f64 * t26114 * t8190;
    (t111143, t111168)
}
