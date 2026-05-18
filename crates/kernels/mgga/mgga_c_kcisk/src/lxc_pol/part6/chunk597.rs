//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 597/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk597<F: Float>(t1537: F, t8365: F, t4463: F, t8349: F, t1212: F, t7802: F, t7819: F, t3725: F, t1529: F, t1542: F, t2293: F, t2297: F, t4436: F, t4461: F, t4471: F, t4478: F, t516: F, t6518: F, t6549: F, t7750: F, t7752: F, t7756: F, t7788: F, t7791: F, t7797: F, t8344: F, t8350: F) -> (F, F, F, F, F, F) {
    let t8366 = t8365 * t1537;
    let t8369 = t8349 * t4463;
    let t8375 = t7802 * t1212;
    let t8378 = t7819 * t1212;
    let t8381 = t7802 * t3725;
    let t8384 = -F::new(0.3109e-1) * t8344 * t516 + F::new(2.0) * t6518 * t2293 - F::new(2.0) * t4436 * t8350 + F::new(1.0) * t1529 * t8366 + F::new(0.32164683177870697974e2) * t4461 * t8369 + t7750 - t7752 + t7756 - t7788 - t7791 - F::new(0.19751789702565206229e-1) * t7797 + F::new(0.11696446794910408142e1) * t6549 * t2297 - F::new(0.11696446794910408142e1) * t4471 * t8375 + F::new(0.58482233974552040708e0) * t1542 * t8378 + F::new(0.17315755899375863299e2) * t4478 * t8381;
    (t8366, t8369, t8375, t8378, t8381, t8384)
}
