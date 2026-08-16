//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 596/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk596(t10514: f64, t6914: f64, t10513: f64, t6711: f64, t2487: f64, t204: f64, t587: f64, t1: f64, t1559: f64, t106: f64, t544: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10516 = 0.62115540045351614476e2_f64 * t6914 * t10514;
    let t10517 = t6711 * t10513;
    let t10519 = 0.43710935587469654631e2_f64 * t2487 * t10517;
    let t10520 = t204 * t10513;
    let t10522 = 0.92023022289409799224e1_f64 * t587 * t10520;
    let t10523 = t1559 * t1;
    let t10524 = t10523 * t106;
    let t10525 = t544 * t10524;
    (t10516, t10519, t10522, t10523, t10524, t10525)
}
