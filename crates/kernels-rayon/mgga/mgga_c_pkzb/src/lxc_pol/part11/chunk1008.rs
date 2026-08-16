//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1008/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1008(t1147: f64, t3702: f64, t10757: f64, t10779: f64, t10782: f64, t10785: f64, t10930: f64, t10951: f64, t10954: f64, t10962: f64, t10966: f64, t10970: f64, t10972: f64, t11104: f64, t135: f64, t273: f64, t6065: f64, t805: f64) -> (f64, f64) {
    let t11108 = t3702 * t1147;
    let t11113 = t11104 * t135 * t273 * t805 + 2.0_f64 * t11108 * t135 * t273 * t6065 + t10757 - t10779 - t10782 + t10785 + t10930 - t10951 - t10954 - t10962 + t10966 - t10970 - t10972;
    (t11108, t11113)
}
