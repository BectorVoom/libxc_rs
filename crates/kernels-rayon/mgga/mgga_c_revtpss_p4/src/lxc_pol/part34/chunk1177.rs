//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1177/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1177(t30100: f64, t7301: f64, t1882: f64, t1903: f64, t543: f64, t25931: f64, t2030: f64, t213: f64, t25930: f64, t26040: f64, t26043: f64, t26058: f64, t26071: f64, t27837: f64, t27966: f64, t27969: f64, t27987: f64, t27990: f64, t27992: f64, t30071: f64, t30074: f64, t30082: f64, t30089: f64, t30096: f64, t561: f64, t6896: f64, t7279: f64, t7295: f64, t7917: f64, t7926: f64, t7930: f64) -> (f64, f64, f64, f64) {
    let t30101 = t7301 * t30100;
    let t30105 = t1903 * t1882 * t543;
    let t30106 = t25931 * t30105;
    let t30109 = 0.10975748638225852664e-1_f64 * t27966 + 0.19514881078765566038e-1_f64 * t27969 + 0.13170898365871023197e1_f64 * t7279 * t6896 - t26040 - 0.4336814094102599731e0_f64 * t30071 * t2030 + t26043 - t26058 + 0.65854491829355115987e0_f64 * t213 * t30074 * t561 + 0.8673628188205199462e0_f64 * t27837 * t7926 - 0.8673628188205199462e0_f64 * t7295 * t30082 - 0.8673628188205199462e0_f64 * t7917 * t7930 - 0.10975748638225852664e-1_f64 * t27987 + 0.4336814094102599731e0_f64 * t7295 * t30089 - 0.14456046980341999104e-1_f64 * t27990 + 0.25702851531048074406e-1_f64 * t27992 + 0.4336814094102599731e0_f64 * t7295 * t30096 + 0.8673628188205199462e0_f64 * t7295 * t30101 - t26071 - 0.17347256376410398924e1_f64 * t25930 * t30106;
    (t30101, t30105, t30106, t30109)
}
