//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1008/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1008<F: Float>(t10845: F, t1233: F, t13798: F, t13801: F, t13805: F, t13812: F, t13819: F, t13823: F, t13827: F, t13854: F, t13860: F, t13866: F, t13974: F, t13977: F, t14028: F, t1694: F, t187: F, t3008: F, t3035: F, t3600: F, t4760: F, t5261: F) -> F {
    let t15291 = -F::new(0.17315755899375863299e2) * t1233 * t13812 + F::new(0.23392893589820816284e1) * t1233 * t13823 - F::new(0.35089340384731224426e1) * t1233 * t13860 - F::new(0.1025389702100779493e4) * t1233 * t13819 + F::new(0.11696446794910408142e1) * t1233 * t13977 - t13798 - t13801 + t13805 + F::new(0.19751789702565206229e-1) * t187 * t13974 + F::new(0.1038945353962551798e3) * t1233 * t13827 - F::new(0.58482233974552040708e0) * t10845 * t1694 - F::new(0.11696446794910408142e1) * t3600 * t4760 + F::new(0.11696446794910408142e1) * t5261 * t3008 - t13854 - F::new(0.58482233974552040708e0) * t1233 * t14028 - F::new(0.17315755899375863299e2) * t5261 * t3035 - t13866;
    t15291
}
