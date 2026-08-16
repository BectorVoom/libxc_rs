//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 433/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk433(t240: f64, t5: f64, t1091: f64, t1094: f64, t2849: f64, t2853: f64, t2860: f64, t2882: f64, t2890: f64, t2892: f64, t2896: f64, t2903: f64, t2914: f64, t2922: f64, t3353: f64) -> (f64, f64) {
    let t3357 = t240 * t5;
    let t3366 = -t2849 - t2853 - t2860 + t2882 + t2890 + t240 * t3353 + 0.24415406715670879921e-3_f64 * t1091 * t2892 + 0.10843580882781524214e-1_f64 * t3357 * t2896 + 0.11696446794910408142e1_f64 * t1094 * t2903 - 0.58482233974552040708e0_f64 * t1094 * t2914 - 0.17315755899375863299e2_f64 * t1094 * t2922;
    (t3357, t3366)
}
