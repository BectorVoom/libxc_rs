//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 609/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk609<F: Float>(t3334: F, t871: F, t3113: F, t984: F, t10318: F, t2321: F, t9074: F, t10268: F, t4261: F, t10122: F, t883: F, t2325: F, t882: F, t10166: F, t3129: F, t9439: F, t986: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t12784 = t3334 * t871;
    let t12785 = t984 * t3113;
    let t12797 = t10318 * t2321;
    let t12798 = t9074 * t12797;
    let t12803 = t4261 * t10268;
    let t12804 = t9074 * t12803;
    let t12819 = t883 * t10122;
    let t12820 = t2325 * t12819;
    let t12821 = t882 * t12820;
    let t12830 = t10166 * t3129;
    let t12831 = t9074 * t12830;
    let t12938 = t9439 * t986;
    (t12784, t12785, t12797, t12798, t12803, t12804, t12820, t12821, t12830, t12831, t12938)
}
