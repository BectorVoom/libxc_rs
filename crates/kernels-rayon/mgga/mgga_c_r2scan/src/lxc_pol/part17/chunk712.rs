//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 712/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk712(t1882: f64, t652: f64, t621: f64, t650: f64, t226: f64, t5270: f64, t1835: f64, t720: f64, t1818: f64, t1821: f64, t219: f64, t225: f64, t5317: f64) -> (f64, f64, f64, f64, f64) {
    let t5706 = t1882 * t652;
    let t5709 = 0.48245938496077605201e2_f64 * t650 * t5706 * t621;
    let t5710 = t226 * t5270;
    let t5714 = t1835 * t5270 * t720;
    let t5717 = t1818 * t5270 * t1821;
    let t5720 = t219 * t5317 * t225;
    (t5709, t5710, t5714, t5717, t5720)
}
