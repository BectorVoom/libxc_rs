//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1399/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1399<F: Float>(t22279: F, t4012: F, t828: F, t1388: F, t14013: F, t14024: F, t1410: F, t22179: F, t22183: F, t22255: F, t22260: F, t22264: F, t22268: F, t22271: F, t22276: F, t5671: F, t9953: F) -> F {
    let t22281 = t4012 * t828 * t22279;
    let t22284 = -F::new(0.36143185997963725434e-4) * t14013 + F::new(0.10003937560882938627e-2) * t22179 + F::new(0.25410001404642664113e-4) * t22183 - F::new(0.21437009059034868486e-3) * t1388 * t22255 - F::new(0.12705000702321332056e-4) * t22260 - F::new(0.57165357490759649296e-4) * t22264 - F::new(0.12705000702321332056e-4) * t22268 - t14024 - t9953 + F::new(0.42874018118069736972e-3) * t5671 * t22271 - F::new(0.25724410870841842183e-1) * t1410 * t22276 + F::new(0.85748036236139473944e-2) * t1410 * t22281;
    t22284
}
