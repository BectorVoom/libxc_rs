//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 795/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk795(t23164: f64, t25316: f64, t1519: f64, t234: f64, t776: f64, t6637: f64, t6552: f64, t1894: f64, t4265: f64, t214: f64, t1880: f64, t23237: f64, t7479: f64) -> (f64, f64, f64, f64) {
    let t25317 = t23164 * t25316;
    let t25319 = t234 * t1519;
    let t25320 = t25319 * t776;
    let t25321 = t6637 * t25320;
    let t25322 = t6552 * t25321;
    let t25324 = t1894 * t4265;
    let t25325 = t214 * t25324;
    let t25326 = t1880 * t25325;
    let t25338 = t23237 * t7479;
    (t25317, t25322, t25326, t25338)
}
