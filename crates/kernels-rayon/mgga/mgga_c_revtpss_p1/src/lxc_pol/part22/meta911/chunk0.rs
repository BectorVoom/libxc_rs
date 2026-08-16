//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3115/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3115(t11223: f64, t16088: f64, t380: f64, t1041: f64, t16185: f64, t3172: f64, t1062: f64, t42261: f64, t11710: f64, t15974: f64, t4899: f64, t11866: f64, t15794: f64) -> (f64, f64, f64, f64, f64) {
    let t54857 = t11223 * t380 * t16088;
    let t54869 = t1041 * t3172 * t16185;
    let t54899 = t42261 * t1062;
    let t54907 = t4899 * t11710 * t15974;
    let t54914 = t11866 * t15794;
    (t54857, t54869, t54899, t54907, t54914)
}
