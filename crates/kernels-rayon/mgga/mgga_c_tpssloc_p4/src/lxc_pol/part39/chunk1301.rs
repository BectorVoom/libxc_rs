//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1301/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1301(t30071: f64, t510: f64, t2199: f64, t3652: f64, t574: f64, t1393: f64, t8189: f64, t1268: f64, t12734: f64, t12739: f64, t12823: f64, t2200: f64, t2202: f64, t2314: f64, t30035: f64, t30038: f64, t4034: f64, t5113: f64, t652: f64, t8176: f64, t8190: f64, t8194: f64, t8196: f64, t9348: f64) -> (f64, f64, f64, f64, f64) {
    let t30072 = t510 * t30071;
    let t30085 = t3652 * t2199;
    let t30088 = t30071 * t574;
    let t30091 = t8189 * t1393;
    let t30094 = 2.0_f64 * t1268 * t30035 + 2.0_f64 * t1268 * t30088 + 4.0_f64 * t1268 * t30091 - 4.0_f64 * t12734 * t2200 + 4.0_f64 * t12734 * t2202 + 2.0_f64 * t12739 * t2202 - 2.0_f64 * t12823 * t2200 - 2.0_f64 * t2200 * t9348 + 2.0_f64 * t2202 * t9348 - 4.0_f64 * t2314 * t8176 - 4.0_f64 * t2314 * t8190 + 4.0_f64 * t2314 * t8194 + 4.0_f64 * t2314 * t8196 - 4.0_f64 * t30038 * t652 - 2.0_f64 * t30072 * t652 - 2.0_f64 * t30085 * t652 - 4.0_f64 * t4034 * t8176 - 4.0_f64 * t4034 * t8190 + 4.0_f64 * t5113 * t8194 + 4.0_f64 * t5113 * t8196;
    (t30072, t30085, t30088, t30091, t30094)
}
