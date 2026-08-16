//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1003/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1003<F: Float>(t11770: F, t2201: F, t3319: F, t2842: F, t3281: F, t10803: F, t10812: F, t10819: F, t10835: F, t10839: F, t11758: F, t11762: F, t11766: F, t11768: F) -> F {
    let t11772 = t2201 * t3319 * t11770;
    let t11774 = t3281 * t2842;
    let t11779 = F::cast_from(0.27439371595564631661e-2_f64) * t11758 + F::cast_from(0.23287303101564395623e-1_f64) * t11762 - F::cast_from(0.69861909304693186867e-1_f64) * t11766 - F::cast_from(0.48787202696913915093e-2_f64) * t11768 - F::cast_from(0.23287303101564395623e-1_f64) * t11772 + F::cast_from(0.54878743191129263322e-2_f64) * t11774 + F::cast_from(0.54878743191129263322e-2_f64) * t10803 + F::cast_from(0.11557628986739024751e0_f64) * t10812 - t10819 + t10835 - F::cast_from(0.11557628986739024751e0_f64) * t10839;
    t11779
}
