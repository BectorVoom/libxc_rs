//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1056/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1056<F: Float>(t17933: F, t7333: F, t2572: F, t7401: F, t11731: F, t9090: F, t1953: F, t9078: F, t741: F, t1871: F, t9014: F, t1937: F, t1931: F, t9058: F, t24019: F, t5322: F, sigma2: F) -> (F, F, F, F, F, F, F, F, F) {
    let t24193 = t17933 * t7333;
    let t24195 = t7401 * t2572;
    let t24197 = t11731 * t9090;
    let t24199 = t9078 * t1953;
    let t24200 = t741 * t24199;
    let t24202 = t9014 * t1871;
    let t24203 = t24202 * sigma2;
    let t24204 = t24203 * t1937;
    let t24206 = t1931 * t9058;
    let t24208 = t5322 * t24019;
    (t24193, t24195, t24197, t24199, t24200, t24202, t24204, t24206, t24208)
}
