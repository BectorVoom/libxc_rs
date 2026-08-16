//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 923/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk923(t242: f64, t6943: f64, t1336: f64, t1878: f64, t557: f64, t556: f64, t598: f64, t281: f64, t6931: f64, t1351: f64, t22705: f64, t236: f64, t550: f64) -> (f64, f64, f64, f64, f64) {
    let t22832 = t6943 * t242;
    let t22833 = t1336 * t22832;
    let t22839 = t1878 * t557;
    let t22842 = t556 * t556;
    let t22843 = 1.0_f64 / t22842;
    let t22844 = t598 * t22843;
    let t22852 = t6931 * t281;
    let t22855 = t22705 * t236 * t1351 * t550;
    (t22833, t22839, t22844, t22852, t22855)
}
