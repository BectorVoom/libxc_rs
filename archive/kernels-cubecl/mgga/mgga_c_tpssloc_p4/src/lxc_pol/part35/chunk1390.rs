//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1390/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1390<F: Float>(t106667: F, t106716: F, t1441: F, t5493: F, t1874: F, t20347: F, t89: F, t28030: F, t7461: F, t20563: F, t24995: F, t8945: F) -> (F, F, F, F, F, F) {
    let t106717 = t106667 + t106716;
    let t106731 = t1441 * t5493;
    let t106733 = F::cast_from(6.0_f64) * t106731 * t1874;
    let t106734 = t89 * t20347;
    let t106736 = F::cast_from(2.0_f64) * t106734 * t1874;
    let t106738 = F::cast_from(6.0_f64) * t28030 * t7461;
    let t106741 = F::cast_from(18.0_f64) * t24995 * t8945 * t20563;
    (t106717, t106731, t106733, t106736, t106738, t106741)
}
