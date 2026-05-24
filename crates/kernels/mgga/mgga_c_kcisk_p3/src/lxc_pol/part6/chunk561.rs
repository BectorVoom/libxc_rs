//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 561/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk561<F: Float>(t1203: F, t1212: F, t7819: F, t3722: F, t7802: F, t3725: F, t1201: F, t2107: F, t45: F, t5765: F, t7750: F, t7752: F, t7756: F, t7788: F, t7791: F, t7797: F, t7804: F) -> (F, F, F) {
    let t7821 = t1203 * t7819 * t1212;
    let t7824 = t3722 * t7802;
    let t7825 = t7824 * t3725;
    let t7828 = -t7750 + t7752 - t7756 + t7788 + t7791 + F::cast_from(0.19751789702565206229e-1_f64) * t45 * t7797 - F::cast_from(0.11696446794910408142e1_f64) * t5765 * t2107 + F::cast_from(0.11696446794910408142e1_f64) * t1201 * t7804 - F::cast_from(0.58482233974552040708e0_f64) * t1201 * t7821 - F::cast_from(0.17315755899375863299e2_f64) * t1201 * t7825;
    (t7821, t7825, t7828)
}
