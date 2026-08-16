//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1726/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1726<F: Float>(t22279: F, t4012: F, t828: F, t1388: F, t14013: F, t14024: F, t1410: F, t22179: F, t22183: F, t22255: F, t22260: F, t22264: F, t22268: F, t22271: F, t22276: F, t5671: F, t9953: F) -> (F, F) {
    let t22281 = t4012 * t828 * t22279;
    let t22284 = -F::cast_from(0.36143185997963725434e-4_f64) * t14013 + F::cast_from(0.10003937560882938627e-2_f64) * t22179 + F::cast_from(0.25410001404642664113e-4_f64) * t22183 - F::cast_from(0.21437009059034868486e-3_f64) * t1388 * t22255 - F::cast_from(0.12705000702321332056e-4_f64) * t22260 - F::cast_from(0.57165357490759649296e-4_f64) * t22264 - F::cast_from(0.12705000702321332056e-4_f64) * t22268 - t14024 - t9953 + F::cast_from(0.42874018118069736972e-3_f64) * t5671 * t22271 - F::cast_from(0.25724410870841842183e-1_f64) * t1410 * t22276 + F::cast_from(0.85748036236139473944e-2_f64) * t1410 * t22281;
    (t22281, t22284)
}
