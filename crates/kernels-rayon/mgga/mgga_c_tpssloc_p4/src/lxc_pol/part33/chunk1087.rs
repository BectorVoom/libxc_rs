//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1087/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1087(t1878: f64, t557: f64, t556: f64, t598: f64, t213: f64, t281: f64, t6931: f64, t2003: f64, t3862: f64, t1887: f64, t22715: f64, t534: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t22839 = t1878 * t557;
    let t22842 = t556 * t556;
    let t22843 = 1.0_f64 / t22842;
    let t22844 = t598 * t22843;
    let t22845 = t22844 * t213;
    let t22852 = t6931 * t281;
    let t22858 = t2003 * t3862;
    let t22859 = 119.0_f64 / 6912.0_f64 * t22858;
    let t22863 = t22715 * t534 * t1887;
    (t22839, t22842, t22843, t22844, t22845, t22852, t22859, t22863)
}
