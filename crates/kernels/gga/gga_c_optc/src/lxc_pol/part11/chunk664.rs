//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 664/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk664<F: Float>(t1796: F, t6739: F, t1990: F, t509: F, t1772: F, t603: F, t1994: F, t171: F, t1974: F, t2045: F, t592: F, t2020: F, t2029: F, t2042: F, t1867: F, t6405: F, t6407: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t6741 = 0.16265371324172286321e-1 * t1796 * t6739;
    let t6745 = t509 * t1990;
    let t6747 = 0.32530742648344572643e-1 * t1796 * t6745;
    let t6748 = t1772 * t603;
    let t6750 = 0.21687161765563048428e-1 * t1796 * t6748;
    let t6751 = t509 * t1994;
    let t6753 = 0.48159446095139119799e0 * t1796 * t6751;
    let t6766 = 1.0 / t1974 / t171;
    let t6770 = t2045 * t592;
    let t6771 = 36.0 * t6770;
    let t6799 = t2020 * t2029;
    let t6811 = 60.0 * t2042 * t592;
    let t6814 = t6405 * t6407 * t1867;
    (t6741, t6745, t6747, t6748, t6750, t6751, t6753, t6766, t6771, t6799, t6811, t6814)
}
