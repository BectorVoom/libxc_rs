//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2709/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2709(t1388: f64, t14013: f64, t14024: f64, t1410: f64, t22179: f64, t22183: f64, t22255: f64, t22260: f64, t22264: f64, t22268: f64, t22271: f64, t22276: f64, t22281: f64, t5671: f64, t9953: f64) -> f64 {
    let t22284 = -0.36143185997963725434e-4_f64 * t14013 + 0.10003937560882938627e-2_f64 * t22179 + 0.25410001404642664113e-4_f64 * t22183 - 0.21437009059034868486e-3_f64 * t1388 * t22255 - 0.12705000702321332056e-4_f64 * t22260 - 0.57165357490759649296e-4_f64 * t22264 - 0.12705000702321332056e-4_f64 * t22268 - t14024 - t9953 + 0.42874018118069736972e-3_f64 * t5671 * t22271 - 0.25724410870841842183e-1_f64 * t1410 * t22276 + 0.85748036236139473944e-2_f64 * t1410 * t22281;
    t22284
}
