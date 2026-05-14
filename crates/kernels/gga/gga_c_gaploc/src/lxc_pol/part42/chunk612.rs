//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 612/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk612<F: Float>(t10007: F, t935: F, t9438: F, t825: F, t10012: F, t2684: F, t3334: F, t871: F, t10318: F, t2321: F, t9074: F, t10268: F, t4261: F, t10166: F, t3129: F, t1531: F, t2876: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t12691 = t10007 * t935;
    let t12692 = t9438 * t12691;
    let t12693 = t825 * t12692;
    let t12704 = t10012 * t935;
    let t12705 = t9438 * t12704;
    let t12706 = t2684 * t12705;
    let t12784 = t3334 * t871;
    let t12797 = t10318 * t2321;
    let t12798 = t9074 * t12797;
    let t12803 = t4261 * t10268;
    let t12804 = t9074 * t12803;
    let t12830 = t10166 * t3129;
    let t12831 = t9074 * t12830;
    let t12881 = t2876 * t1531;
    (t12691, t12692, t12693, t12704, t12705, t12706, t12784, t12797, t12798, t12803, t12804, t12830, t12831, t12881)
}
