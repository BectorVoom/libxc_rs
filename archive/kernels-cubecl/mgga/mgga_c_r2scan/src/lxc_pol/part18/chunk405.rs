//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 405/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk405<F: Float>(t1835: F, t224: F, t1691: F, t720: F, t234: F, t712: F, t717: F, t749: F, t732: F, t741: F, t750: F, t625: F, t626: F, t645: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t1836 = t1835 * t224;
    let t1837 = t720 * t1691;
    let t1838 = t1836 * t1837;
    let t1840 = F::cast_from(0.10389515463408878255e3_f64) * t234 * t1838;
    let t1841 = t717 * t712;
    let t1842 = t1841 * t749;
    let t1844 = F::cast_from(0.34631718211362927518e2_f64) * t234 * t1842;
    let t1845 = t732 * t741;
    let t1847 = t732 * t750;
    let t1851 = F::cast_from(0.35616666666666666666e-1_f64) * t625 * t626 * t645;
    (t1836, t1837, t1838, t1840, t1841, t1842, t1844, t1845, t1847, t1851)
}
