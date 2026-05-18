//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1186/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1186<F: Float>(t30100: F, t7301: F, t1882: F, t1903: F, t543: F, t25931: F, t2030: F, t213: F, t25930: F, t26040: F, t26043: F, t26058: F, t26071: F, t27837: F, t27966: F, t27969: F, t27987: F, t27990: F, t27992: F, t30071: F, t30074: F, t30082: F, t30089: F, t30096: F, t561: F, t6896: F, t7279: F, t7295: F, t7917: F, t7926: F, t7930: F) -> (F, F, F, F) {
    let t30101 = t7301 * t30100;
    let t30105 = t1903 * t1882 * t543;
    let t30106 = t25931 * t30105;
    let t30109 = F::new(0.10975748638225852664e-1) * t27966 + F::new(0.19514881078765566038e-1) * t27969 + F::new(0.13170898365871023197e1) * t7279 * t6896 - t26040 - F::new(0.4336814094102599731e0) * t30071 * t2030 + t26043 - t26058 + F::new(0.65854491829355115987e0) * t213 * t30074 * t561 + F::new(0.8673628188205199462e0) * t27837 * t7926 - F::new(0.8673628188205199462e0) * t7295 * t30082 - F::new(0.8673628188205199462e0) * t7917 * t7930 - F::new(0.10975748638225852664e-1) * t27987 + F::new(0.4336814094102599731e0) * t7295 * t30089 - F::new(0.14456046980341999104e-1) * t27990 + F::new(0.25702851531048074406e-1) * t27992 + F::new(0.4336814094102599731e0) * t7295 * t30096 + F::new(0.8673628188205199462e0) * t7295 * t30101 - t26071 - F::new(0.17347256376410398924e1) * t25930 * t30106;
    (t30101, t30105, t30106, t30109)
}
