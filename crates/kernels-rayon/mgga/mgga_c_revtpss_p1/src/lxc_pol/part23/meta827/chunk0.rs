//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2683/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2683(t6244: f64, t905: f64, t11774: f64, t4782: f64, t53391: f64, t1011: f64, t15993: f64, t18909: f64, t11933: f64, t19976: f64, t3115: f64, t42793: f64, t6272: f64) -> (f64, f64, f64, f64, f64) {
    let t66966 = t6244 * t905;
    let t66972 = t11774 * t53391 * t4782;
    let t66981 = t1011 * t15993 * t18909;
    let t67006 = t11933 * t19976;
    let t67015 = t3115 * t42793 * t6272;
    (t66966, t66972, t66981, t67006, t67015)
}
