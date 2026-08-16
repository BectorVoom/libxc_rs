//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 603/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk603(t5790: f64, t938: f64, t1294: f64, t1602: f64, t1300: f64, t1701: f64, t2035: f64, t22513: f64, t22522: f64, t22541: f64, t22583: f64, t22590: f64, t22591: f64, t22826: f64, t22842: f64, t25670: f64, t25676: f64, t25680: f64, t25685: f64, t25689: f64, t25695: f64, t25699: f64, t25704: f64, t25708: f64, t25710: f64, t25715: f64, t25719: f64, t25722: f64, t25726: f64, t3061: f64, t3067: f64, t5569: f64, t5570: f64, t5579: f64, t5598: f64, t7867: f64, t7889: f64) -> (f64, f64, f64) {
    let t25730 = t5790 * t938;
    let t25734 = t1602 * t1294;
    let t25739 = 0.38306165027777777778e-1_f64 * t5598 * t5579 * t25670 - 0.11854761295685025975e-1_f64 * t22842 * t25676 + 0.44455354858818847408e-2_f64 * t22590 * t22591 * t25680 - 0.44455354858818847408e-2_f64 * t7889 * t25685 + 0.7423383944657264111e-4_f64 * t22583 * t25689 + 0.74233839446572641111e-4_f64 * t22583 * t25695 - 0.12768721675925925926e-1_f64 * t22541 * t5570 * t25699 + 0.12768721675925925926e-1_f64 * t22522 * t5570 * t25704 + 0.12768721675925925926e-1_f64 * t25708 * t25710 - 0.15137014751914110597e-3_f64 * t22513 * t25715 - 0.85124811172839506173e-2_f64 * t25708 * t25719 + 0.22270151833971792333e-3_f64 * t5569 * t5570 * t25722 + 0.22227677429409423704e-2_f64 * t1300 * t1701 * t25726 + 0.52700762016626893448e-4_f64 * t7867 * t2035 * t25730 + 0.38731446812548799881e-3_f64 * t25734 * t3061 + 0.23254900946437792e-1_f64 * t22826 * t3067;
    (t25730, t25734, t25739)
}
