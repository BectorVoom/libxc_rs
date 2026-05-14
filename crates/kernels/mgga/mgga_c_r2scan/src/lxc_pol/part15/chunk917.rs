//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 917/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk917<F: Float>(t10894: F, t927: F, t787: F, t978: F, t3320: F, t783: F, t910: F, t2207: F, t3319: F, t10856: F, t2605: F, t938: F, t2201: F, t2842: F, t3281: F, t10803: F, t10812: F, t10819: F, t10835: F, t10839: F) -> (F, F, F, F) {
    let t11758 = t10894 * t927;
    let t11760 = t978 * t787;
    let t11762 = t783 * t11760 * t3320;
    let t11764 = t3320 * t910;
    let t11766 = t2207 * t3319 * t11764;
    let t11768 = t10856 * t2605;
    let t11770 = t3320 * t938;
    let t11772 = t2201 * t3319 * t11770;
    let t11774 = t3281 * t2842;
    let t11779 = 0.27439371595564631661e-2 * t11758 + 0.23287303101564395623e-1 * t11762 - 0.69861909304693186867e-1 * t11766 - 0.48787202696913915093e-2 * t11768 - 0.23287303101564395623e-1 * t11772 + 0.54878743191129263322e-2 * t11774 + 0.54878743191129263322e-2 * t10803 + 0.11557628986739024751e0 * t10812 - t10819 + t10835 - 0.11557628986739024751e0 * t10839;
    (t11760, t11764, t11770, t11779)
}
