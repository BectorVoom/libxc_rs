//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 704/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk704(t13472: f64, t11172: f64, t874: f64, t1445: f64, t597: f64, t12991: f64, t12997: f64, t12961: f64, t12966: f64, t12988: f64, t12994: f64, t13458: f64, t13463: f64, t13466: f64, t13469: f64, t574: f64) -> (f64, f64, f64) {
    let t13473 = 0.19171462976960374838e0_f64 * t13472;
    let t13474 = t11172 * t874;
    let t13475 = t1445 * t13474;
    let t13477 = 0.43710935587469654631e2_f64 * t597 * t13475;
    let t13478 = 0.59584149919750711116e-1_f64 * t12991;
    let t13480 = 0.11916829983950142223e0_f64 * t12997;
    let t13481 = -0.23005755572352449806e1_f64 * t574 * t13458 + 0.38342925953920749677e1_f64 * t12961 - 0.76685851907841499353e0_f64 * t12966 - t13463 + 0.63904876589867916128e-1_f64 * t12988 - 0.38342925953920749677e0_f64 * t13466 - 0.57514388930881124515e0_f64 * t13469 + t13473 + t13477 + t13478 + 0.76685851907841499353e0_f64 * t12994 + t13480;
    (t13474, t13475, t13481)
}
