//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 433/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk433(t1835: f64, t224: f64, t1691: f64, t720: f64, t234: f64, t712: f64, t717: f64, t749: f64, t732: f64, t741: f64, t750: f64, t625: f64, t626: f64, t645: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1836 = t1835 * t224;
    let t1837 = t720 * t1691;
    let t1838 = t1836 * t1837;
    let t1840 = 0.10389515463408878255e3_f64 * t234 * t1838;
    let t1841 = t717 * t712;
    let t1842 = t1841 * t749;
    let t1844 = 0.34631718211362927518e2_f64 * t234 * t1842;
    let t1845 = t732 * t741;
    let t1847 = t732 * t750;
    let t1851 = 0.35616666666666666666e-1_f64 * t625 * t626 * t645;
    (t1836, t1837, t1838, t1840, t1841, t1842, t1844, t1845, t1847, t1851)
}
