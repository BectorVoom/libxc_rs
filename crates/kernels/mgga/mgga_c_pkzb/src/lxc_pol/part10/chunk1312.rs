//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1312/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1312<F: Float>(t1893: F, t1899: F, t3525: F, t1856: F, t3554: F, t5802: F, t1084: F, t1855: F, t7443: F, t5776: F, t17541: F, t9235: F, t3551: F, t9228: F, t3550: F, t5804: F) -> (F, F, F, F, F, F, F, F, F) {
    let t25918 = 6.0 * t1899 * t3525 * t1893;
    let t25921 = 0.57895126195293126241e3 * t5802 * t3554 * t1856;
    let t25924 = 4.0 * t1855 * t1084 * t7443;
    let t25927 = 0.96491876992155210402e2 * t5776 * t3554 * t1893;
    let t25930 = 0.62071215503128080361e4 * t17541 * t9235 * t1856;
    let t25933 = 2.0 * t1855 * t3551 * t1893;
    let t25936 = 0.96491876992155210402e2 * t5776 * t9228 * t1856;
    let t25939 = 0.16081979498692535067e2 * t1899 * t9228 * t1893;
    let t25940 = t3550 * t5804;
    (t25918, t25921, t25924, t25927, t25930, t25933, t25936, t25939, t25940)
}
