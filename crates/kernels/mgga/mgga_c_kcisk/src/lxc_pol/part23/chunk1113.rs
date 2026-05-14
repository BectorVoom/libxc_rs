//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1113/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1113<F: Float>(t31910: F, t9304: F, t2677: F, t3050: F, t3052: F, t3934: F, t933: F, t114: F, t3042: F, t856: F, t2679: F, t2927: F, t9314: F, t912: F, t918: F, t3063: F, t9305: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t31911 = t9304 * t31910;
    let t31913 = t2677 * t31910;
    let t31917 = t3934 * t3050 * t933 * t3052;
    let t31918 = t2677 * t31917;
    let t31920 = t114 * t3042;
    let t31921 = t856 * t31920;
    let t31922 = t31921 * t2679;
    let t31924 = t2927 * t9314;
    let t31925 = t31924 * t2679;
    let t31927 = t912 * t918;
    let t31928 = t856 * t31927;
    let t31929 = t31928 * t2679;
    let t31932 = t3934 * t9305 * t3063;
    (t31911, t31913, t31917, t31918, t31920, t31921, t31922, t31924, t31925, t31927, t31928, t31929, t31932)
}
