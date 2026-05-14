//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 669/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk669<F: Float>(t2016: F, t2025: F, t2630: F, t2634: F, t5468: F, t5479: F, t5481: F, t7575: F, t7578: F, t7581: F, t7586: F, t7589: F, t7591: F, t788: F, t2637: F, t4998: F) -> (F, F) {
    let t7598 = 0.2698618307426597582e-1 * t7575 * t788 + 0.89953943580886586067e-2 * t7578 + 0.89953943580886586067e-2 * t7581 * t2016 - 0.2698618307426597582e-1 * t2630 * t2025 - 0.71963154864709268853e-1 * t7586 * t788 - 0.23987718288236422951e-1 * t7589 - 0.23987718288236422951e-1 * t7591 * t2016 + 0.71963154864709268853e-1 * t2634 * t2025 + 0.89953943580886586067e-2 * t5468 - t5479 + 0.29984647860295528689e-2 * t5481;
    let t7602 = t4998 * t2637;
    (t7598, t7602)
}
