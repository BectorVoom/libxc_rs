//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 836/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk836(t1911: f64, t857: f64, t2717: f64, t794: f64, t8331: f64, t6562: f64, t6547: f64, t8332: f64, t23204: f64, t8335: f64, t1902: f64, t214: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t30622 = t857 * t1911;
    let t30633 = t2717 * t1911;
    let t30638 = t794 * t8331;
    let t30640 = 0.82246703342411321825e-2_f64 * t6562 * t30638;
    let t30655 = 0.38381794893125283518e-1_f64 * t6547 * t8332;
    let t30660 = t23204 * t8335;
    let t30662 = 0.82246703342411321825e-2_f64 * t6562 * t30660;
    let t30663 = t214 * t1902;
    (t30622, t30633, t30638, t30640, t30655, t30660, t30662, t30663)
}
