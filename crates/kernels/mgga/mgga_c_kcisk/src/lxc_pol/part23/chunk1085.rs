//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1085/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1085<F: Float>(t1537: F, t21817: F, t14728: F, t14747: F, t1529: F, t1538: F, t1542: F, t19577: F, t19602: F, t21748: F, t21755: F, t21759: F, t21764: F, t2293: F, t2297: F, t4431: F, t4456: F, t4464: F, t4468: F, t4475: F, t4479: F, t6518: F, t6541: F, t6549: F, t6557: F) -> (F,) {
    let t21818 = t21817 * t1537;
    let t21821 = 0.58482233974552040708e0 * t6549 * t4475 + 0.17315755899375863299e2 * t21748 * t4479 + 0.58482233974552040708e0 * t14728 * t2297 + 0.11696446794910408142e1 * t4468 * t6557 + 0.58482233974552040708e0 * t1542 * t21755 - 0.19751789702565206229e-1 * t19577 + 2.0 * t21759 * t1538 + 1.0 * t6518 * t4456 + 0.32164683177870697974e2 * t21764 * t4464 + 1.0 * t14747 * t2293 + 2.0 * t4431 * t6541 + 1.0 * t1529 * t21818 - t19602;
    (t21821,)
}
