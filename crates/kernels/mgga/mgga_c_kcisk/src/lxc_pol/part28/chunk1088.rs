//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1088/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1088<F: Float>(t10886: F, t9172: F, t2013: F, t22294: F, t5497: F, t7246: F, t2023: F, t7715: F, t12234: F, t1775: F, t22289: F, t5486: F, t2063: F, t7638: F, t5491: F, t220: F, t2642: F) -> (F, F, F, F, F, F, F, F) {
    let t24925 = t10886 * t9172;
    let t24926 = t2013 * t24925;
    let t24930 = t5497 * t22294;
    let t24931 = t7246 * t24930;
    let t24934 = t7715 * t2023;
    let t24935 = t12234 * t24934;
    let t24936 = t1775 * t24935;
    let t24939 = t5486 * t22289;
    let t24940 = t1775 * t24939;
    let t24943 = t2063 * t7638;
    let t24944 = t5491 * t24943;
    let t24945 = t1775 * t24944;
    let t24948 = t220 * t2642;
    (t24926, t24931, t24934, t24936, t24940, t24943, t24945, t24948)
}
