//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1231/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1231<F: Float>(t30293: F, t7375: F, t7378: F, t10800: F, t17444: F, t667: F, t2759: F, t9171: F, t17351: F, t17405: F, t17566: F, t20705: F, t25633: F, t25636: F, t25734: F, t25740: F, t25747: F, t25750: F, t25767: F, t30284: F, t30287: F, t30289: F, t30291: F) -> (F, F, F, F, F) {
    let t30294 = t7375 * t30293;
    let t30296 = t7378 * t30293;
    let t30309 = t17444 * t10800 * t667;
    let t30311 = t9171 * t2759;
    let t30313 = -F::new(0.7302814814814814815e0) * t17405 - F::new(0.27903555555555555556e1) * t20705 + F::new(0.1898925e1) * t30289 + F::new(0.3071625e0) * t30291 + F::new(0.427258125e1) * t30294 - F::new(0.230371875e0) * t30296 + t17566 - F::new(0.93011851851851851854e0) * t17351 + F::new(0.11958666666666666667e1) * t25633 - F::new(0.89690000000000000001e0) * t25636 + F::new(0.82156666666666666665e0) * t25734 - F::new(0.29896666666666666667e0) * t30284 + F::new(0.8969e0) * t30287 - F::new(0.98587999999999999998e0) * t25740 - F::new(0.49293999999999999999e0) * t25747 - F::new(0.49293999999999999999e0) * t25750 + F::new(0.82156666666666666665e0) * t25767 + F::new(0.1151859375e0) * t30309 - F::new(0.230371875e0) * t30311;
    (t30294, t30296, t30309, t30311, t30313)
}
