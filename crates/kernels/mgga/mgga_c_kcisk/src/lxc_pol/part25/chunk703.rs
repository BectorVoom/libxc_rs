//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 703/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk703<F: Float>(t7574: F, t781: F, t2009: F, t2630: F, t2629: F, t1772: F, t2004: F, t2633: F, t2634: F, t2012: F, t7218: F, t2016: F, t2025: F, t5468: F, t5479: F, t5481: F, t788: F, sigma2: F) -> (F, F, F, F, F) {
    let t7575 = t7574 * t781;
    let t7578 = t2630 * t2009;
    let t7580 = t2629 * sigma2;
    let t7581 = t7580 * t1772;
    let t7586 = t2004 * t2633;
    let t7589 = t2634 * t2009;
    let t7591 = t2012 * t7218;
    let t7598 = 0.2698618307426597582e-1 * t7575 * t788 + 0.89953943580886586067e-2 * t7578 + 0.89953943580886586067e-2 * t7581 * t2016 - 0.2698618307426597582e-1 * t2630 * t2025 - 0.71963154864709268853e-1 * t7586 * t788 - 0.23987718288236422951e-1 * t7589 - 0.23987718288236422951e-1 * t7591 * t2016 + 0.71963154864709268853e-1 * t2634 * t2025 + 0.89953943580886586067e-2 * t5468 - t5479 + 0.29984647860295528689e-2 * t5481;
    (t7575, t7581, t7586, t7591, t7598)
}
