//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1048/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1048<F: Float>(t18682: F, t695: F, t18681: F, t2634: F, t5483: F, t2005: F, t7624: F, t12261: F, t2643: F, t782: F, t4419: F, t7639: F, t2009: F, t7586: F, t2630: F, t2644: F, t5465: F, t5511: F, t5517: F, t7640: F) -> (F, F) {
    let t18683 = t18682 * t695;
    let t18684 = t18681 * t18683;
    let t18693 = 0.47975436576472845902e-1 * t2634 * t5483;
    let t18699 = 0.17990788716177317213e-1 * t2005 * t7624;
    let t18700 = t12261 * t2643;
    let t18701 = t782 * t18700;
    let t18703 = t4419 * t7639;
    let t18705 = 0.17990788716177317213e-1 * t782 * t18703;
    let t18711 = 0.47975436576472845902e-1 * t7586 * t2009;
    let t18713 = 0.17990788716177317213e-1 * t2630 * t5483;
    let t18714 = 0.5397236614853195164e-1 * t2630 * t5511 + t18693 + 0.71963154864709268853e-1 * t2634 * t5517 - 0.2698618307426597582e-1 * t2630 * t5517 - t18699 + 0.59969295720591057378e-2 * t18701 - t18705 - 0.2698618307426597582e-1 * t5465 * t2644 - 0.5397236614853195164e-1 * t2005 * t7640 - t18711 - t18713;
    (t18684, t18714)
}
