//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1153/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1153(t10089: f64, t3259: f64, t4415: f64, t4416: f64, t3261: f64, t3267: f64, t4462: f64, t12678: f64, t12688: f64, t12690: f64, t12692: f64, t12729: f64, t12730: f64, t12738: f64, t7929: f64, t7932: f64, t7936: f64, t7945: f64, t9839: f64, t9844: f64, t9846: f64, t9848: f64, t9854: f64) -> (f64, f64, f64, f64, f64) {
    let t12892 = t10089 * t3259;
    let t12894 = t4415 * t4416 * t12892;
    let t12898 = t4415 * t4416 * t3261;
    let t12902 = 7.0_f64 / 2304.0_f64 * t3267 * t4462;
    let t12903 = t12678 - t12688 - t12690 + t12692 + t12729 - t9839 + t12730 + t9844 + t9846 - t9848 + t7929 - t7932 - t7936 + t9854 + t7945 - t12738;
    (t12892, t12894, t12898, t12902, t12903)
}
