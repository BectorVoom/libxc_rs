//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1268/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1268<F: Float>(t1353: F, t6836: F, t828: F, t9942: F, t1868: F, t5591: F, t4012: F, t1388: F, t14013: F, t14024: F, t1410: F, t22179: F, t22183: F, t22255: F, t22260: F, t22264: F, t22268: F, t22271: F, t5671: F, t9953: F) -> (F,) {
    let t22274 = t6836 * t1353;
    let t22276 = t9942 * t828 * t22274;
    let t22279 = t1868 * t5591;
    let t22281 = t4012 * t828 * t22279;
    let t22284 = -0.36143185997963725434e-4 * t14013 + 0.10003937560882938627e-2 * t22179 + 0.25410001404642664113e-4 * t22183 - 0.21437009059034868486e-3 * t1388 * t22255 - 0.12705000702321332056e-4 * t22260 - 0.57165357490759649296e-4 * t22264 - 0.12705000702321332056e-4 * t22268 - t14024 - t9953 + 0.42874018118069736972e-3 * t5671 * t22271 - 0.25724410870841842183e-1 * t1410 * t22276 + 0.85748036236139473944e-2 * t1410 * t22281;
    (t22284,)
}
