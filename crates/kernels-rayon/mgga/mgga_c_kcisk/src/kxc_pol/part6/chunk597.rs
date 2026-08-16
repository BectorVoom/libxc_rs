//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 597/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk597(t1537: f64, t8365: f64, t4463: f64, t8349: f64, t1212: f64, t7802: f64, t7819: f64, t3725: f64, t1529: f64, t1542: f64, t2293: f64, t2297: f64, t4436: f64, t4461: f64, t4471: f64, t4478: f64, t516: f64, t6518: f64, t6549: f64, t7750: f64, t7752: f64, t7756: f64, t7788: f64, t7791: f64, t7797: f64, t8344: f64, t8350: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8366 = t8365 * t1537;
    let t8369 = t8349 * t4463;
    let t8375 = t7802 * t1212;
    let t8378 = t7819 * t1212;
    let t8381 = t7802 * t3725;
    let t8384 = -0.3109e-1_f64 * t8344 * t516 + 2.0_f64 * t6518 * t2293 - 2.0_f64 * t4436 * t8350 + 1.0_f64 * t1529 * t8366 + 0.32164683177870697974e2_f64 * t4461 * t8369 + t7750 - t7752 + t7756 - t7788 - t7791 - 0.19751789702565206229e-1_f64 * t7797 + 0.11696446794910408142e1_f64 * t6549 * t2297 - 0.11696446794910408142e1_f64 * t4471 * t8375 + 0.58482233974552040708e0_f64 * t1542 * t8378 + 0.17315755899375863299e2_f64 * t4478 * t8381;
    (t8366, t8369, t8375, t8378, t8381, t8384)
}
