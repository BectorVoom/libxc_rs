//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 704/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk704<F: Float>(t13472: F, t11172: F, t874: F, t1445: F, t597: F, t12991: F, t12997: F, t12961: F, t12966: F, t12988: F, t12994: F, t13458: F, t13463: F, t13466: F, t13469: F, t574: F) -> (F, F, F) {
    let t13473 = F::new(0.19171462976960374838e0) * t13472;
    let t13474 = t11172 * t874;
    let t13475 = t1445 * t13474;
    let t13477 = F::new(0.43710935587469654631e2) * t597 * t13475;
    let t13478 = F::new(0.59584149919750711116e-1) * t12991;
    let t13480 = F::new(0.11916829983950142223e0) * t12997;
    let t13481 = -F::new(0.23005755572352449806e1) * t574 * t13458 + F::new(0.38342925953920749677e1) * t12961 - F::new(0.76685851907841499353e0) * t12966 - t13463 + F::new(0.63904876589867916128e-1) * t12988 - F::new(0.38342925953920749677e0) * t13466 - F::new(0.57514388930881124515e0) * t13469 + t13473 + t13477 + t13478 + F::new(0.76685851907841499353e0) * t12994 + t13480;
    (t13474, t13475, t13481)
}
