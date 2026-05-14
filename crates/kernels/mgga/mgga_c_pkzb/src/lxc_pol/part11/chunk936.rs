//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 936/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk936<F: Float>(t11100: F, t790: F, t11054: F, t11064: F, t11067: F, t1134: F, t1144: F, t307: F, t311: F, t3670: F, t3676: F, t3695: F, t1147: F, t3702: F, t10757: F, t10779: F, t10782: F, t10785: F, t10930: F, t10951: F, t10954: F, t10962: F, t10966: F, t10970: F, t10972: F, t135: F, t273: F, t6065: F, t805: F) -> (F, F, F, F) {
    let t11101 = t790 * t11100;
    let t11104 = 0.65854491829355115987e0 * t11054 * t311 - 0.19756347548806534796e1 * t3670 * t1144 + 0.39512695097613069591e1 * t1134 * t3676 - 0.19756347548806534796e1 * t1134 * t3695 - 0.39512695097613069591e1 * t307 * t11064 + 0.39512695097613069591e1 * t307 * t11067 - 0.65854491829355115987e0 * t307 * t11101;
    let t11108 = t3702 * t1147;
    let t11113 = t11104 * t135 * t273 * t805 + 2.0 * t11108 * t135 * t273 * t6065 + t10757 - t10779 - t10782 + t10785 + t10930 - t10951 - t10954 - t10962 + t10966 - t10970 - t10972;
    (t11101, t11104, t11108, t11113)
}
