//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1428/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1428(t1873: f64, t75784: f64, t28017: f64, t5371: f64, t106951: f64, t1401: f64, t1851: f64, t5456: f64, t16524: f64, t28896: f64, t28899: f64, t33185: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t107566 = 0.135e2_f64 * t75784 * t1873;
    let t107568 = 0.405e2_f64 * t5371 * t28017;
    let t107570 = 0.135e2_f64 * t1401 * t106951;
    let t107571 = t1851 * t5456;
    let t107573 = 81.0_f64 * t107571 * t1873;
    let t107575 = 162.0_f64 * t16524 * t28896;
    let t107577 = 81.0_f64 * t16524 * t28899;
    let t107579 = 81.0_f64 * t33185 * t28899;
    (t107566, t107568, t107570, t107573, t107575, t107577, t107579)
}
