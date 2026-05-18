//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 677/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk677<F: Float>(t10318: F, t2321: F, t9074: F, t10268: F, t4261: F, t10166: F, t3129: F, t1531: F, t2876: F, t9439: F, t986: F, t9438: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12797 = t10318 * t2321;
    let t12798 = t9074 * t12797;
    let t12803 = t4261 * t10268;
    let t12804 = t9074 * t12803;
    let t12830 = t10166 * t3129;
    let t12831 = t9074 * t12830;
    let t12881 = t2876 * t1531;
    let t12938 = t9439 * t986;
    let t12939 = t9438 * t12938;
    (t12797, t12798, t12803, t12804, t12830, t12831, t12881, t12938, t12939)
}
