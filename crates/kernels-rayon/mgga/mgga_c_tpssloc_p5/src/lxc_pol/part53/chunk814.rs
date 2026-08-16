//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 814/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk814(t22833: f64, t5303: f64, t1351: f64, t16311: f64, t3788: f64, t6936: f64, t16306: f64, t550: f64, t1339: f64, t1887: f64, t22839: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26314 = t22833 * t5303;
    let t26318 = t16311 * t1351;
    let t26319 = t3788 * t26318;
    let t26320 = t6936 * t26319;
    let t26322 = t16306 * t550;
    let t26323 = t1339 * t26322;
    let t26324 = t6936 * t26323;
    let t26331 = t22839 * t1887;
    (t26314, t26318, t26320, t26322, t26324, t26331)
}
