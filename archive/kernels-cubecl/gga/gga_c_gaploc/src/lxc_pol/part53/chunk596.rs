//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 596/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk596<F: Float>(t10514: F, t6914: F, t10513: F, t6711: F, t2487: F, t204: F, t587: F, t1: F, t1559: F, t106: F, t544: F) -> (F, F, F, F, F, F) {
    let t10516 = F::cast_from(0.62115540045351614476e2_f64) * t6914 * t10514;
    let t10517 = t6711 * t10513;
    let t10519 = F::cast_from(0.43710935587469654631e2_f64) * t2487 * t10517;
    let t10520 = t204 * t10513;
    let t10522 = F::cast_from(0.92023022289409799224e1_f64) * t587 * t10520;
    let t10523 = t1559 * t1;
    let t10524 = t10523 * t106;
    let t10525 = t544 * t10524;
    (t10516, t10519, t10522, t10523, t10524, t10525)
}
