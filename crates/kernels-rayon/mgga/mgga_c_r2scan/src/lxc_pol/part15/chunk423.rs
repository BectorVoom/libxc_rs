//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 423/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk423(t1743: f64, t219: f64, t225: f64, t234: f64, t1398: f64, t236: f64, t735: f64, t424: f64, t5: f64, t736: f64, t378: f64, t745: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1745 = t219 * t1743 * t225;
    let t1747 = 0.5848223622634646207e0_f64 * t234 * t1745;
    let t1748 = t1398 * t236;
    let t1750 = 0.72290542002011598948e-2_f64 * t735 * t1748;
    let t1751 = t424 * t5;
    let t1752 = t1751 * t736;
    let t1754 = t378 * t745;
    (t1745, t1747, t1748, t1750, t1751, t1752, t1754)
}
