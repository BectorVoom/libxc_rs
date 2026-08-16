//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1003/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1003(t11770: f64, t2201: f64, t3319: f64, t2842: f64, t3281: f64, t10803: f64, t10812: f64, t10819: f64, t10835: f64, t10839: f64, t11758: f64, t11762: f64, t11766: f64, t11768: f64) -> f64 {
    let t11772 = t2201 * t3319 * t11770;
    let t11774 = t3281 * t2842;
    let t11779 = 0.27439371595564631661e-2_f64 * t11758 + 0.23287303101564395623e-1_f64 * t11762 - 0.69861909304693186867e-1_f64 * t11766 - 0.48787202696913915093e-2_f64 * t11768 - 0.23287303101564395623e-1_f64 * t11772 + 0.54878743191129263322e-2_f64 * t11774 + 0.54878743191129263322e-2_f64 * t10803 + 0.11557628986739024751e0_f64 * t10812 - t10819 + t10835 - 0.11557628986739024751e0_f64 * t10839;
    t11779
}
