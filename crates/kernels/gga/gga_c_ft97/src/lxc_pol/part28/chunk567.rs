//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 567/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk567<F: Float>(t3099: F, t5546: F, t5790: F, t938: F, t1294: F, t1602: F, t1300: F, t1701: F, t2035: F, t22513: F, t22522: F, t22541: F, t22583: F, t22590: F, t22591: F, t22826: F, t22842: F, t25670: F, t25676: F, t25680: F, t25685: F, t25689: F, t25695: F, t25699: F, t25704: F, t25708: F, t25710: F, t25715: F, t25719: F, t25722: F, t3061: F, t3067: F, t5569: F, t5570: F, t5579: F, t5598: F, t7867: F, t7889: F) -> (F, F, F, F) {
    let t25726 = t5546 * t3099;
    let t25730 = t5790 * t938;
    let t25734 = t1602 * t1294;
    let t25739 = 0.38306165027777777778e-1 * t5598 * t5579 * t25670 - 0.11854761295685025975e-1 * t22842 * t25676 + 0.44455354858818847408e-2 * t22590 * t22591 * t25680 - 0.44455354858818847408e-2 * t7889 * t25685 + 0.7423383944657264111e-4 * t22583 * t25689 + 0.74233839446572641111e-4 * t22583 * t25695 - 0.12768721675925925926e-1 * t22541 * t5570 * t25699 + 0.12768721675925925926e-1 * t22522 * t5570 * t25704 + 0.12768721675925925926e-1 * t25708 * t25710 - 0.15137014751914110597e-3 * t22513 * t25715 - 0.85124811172839506173e-2 * t25708 * t25719 + 0.22270151833971792333e-3 * t5569 * t5570 * t25722 + 0.22227677429409423704e-2 * t1300 * t1701 * t25726 + 0.52700762016626893448e-4 * t7867 * t2035 * t25730 + 0.38731446812548799881e-3 * t25734 * t3061 + 0.23254900946437792e-1 * t22826 * t3067;
    (t25726, t25730, t25734, t25739)
}
