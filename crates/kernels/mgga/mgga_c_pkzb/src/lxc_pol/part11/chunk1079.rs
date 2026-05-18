//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1079/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1079<F: Float>(t17928: F, t2018: F, t197: F, t2021: F, t294: F, t2029: F, t750: F, t148: F, t616: F, t757: F, t762: F, t288: F, t5950: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t17929 = t17928 * t2018;
    let t17930 = t17929 * t197;
    let t17931 = t2021 * t2021;
    let t17932 = F::new(1.0) / t17931;
    let t17933 = t294 * t17932;
    let t17938 = t2029 * t2029;
    let t17945 = t17928 * t750;
    let t17946 = t17945 * t197;
    let t17955 = t616 * t148;
    let t17957 = t757 * t17955 * t762;
    let t17999 = t17928 / t5950 / t288;
    (t17929, t17930, t17932, t17933, t17938, t17945, t17946, t17955, t17957, t17999)
}
