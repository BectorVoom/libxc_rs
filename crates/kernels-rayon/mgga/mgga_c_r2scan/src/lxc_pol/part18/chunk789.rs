//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 789/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk789(t2755: f64, t406: f64, t1861: f64, t2768: f64, t1860: f64, t1859: f64, t2482: f64, t1862: f64, t2743: f64, t5322: f64, t1838: f64, t963: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7653 = 8.0_f64 * t406 * t2755;
    let t7654 = t2768 * t1861;
    let t7656 = 0.2701041328e0_f64 * t1860 * t7654;
    let t7657 = t1859 * t2482;
    let t7659 = 0.2701041328e0_f64 * t7657 * t1862;
    let t7661 = 0.2701041328e0_f64 * t2743 * t5322;
    let t7662 = t963 * t1838;
    (t7653, t7654, t7656, t7659, t7661, t7662)
}
