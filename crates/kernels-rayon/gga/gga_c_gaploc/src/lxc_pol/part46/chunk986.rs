//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 986/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk986(t10086: f64, t10811: f64, t43508: f64, t7427: f64, t7573: f64, t326: f64, t43486: f64, t825: f64, t43598: f64, t7584: f64, t7585: f64, t43690: f64, t43693: f64, t43695: f64, t43698: f64, t43699: f64, t43703: f64, t43708: f64, t43712: f64, t43716: f64, t43719: f64, t43721: f64, t43723: f64, t43726: f64, t43729: f64, t43731: f64, t43735: f64, t780: f64) -> f64 {
    let t43737 = 0.42900587942220512003e1_f64 * t10811 * t10086;
    let t43740 = 0.62115540045351614476e2_f64 * t7427 * t7573 * t43508;
    let t43743 = 0.18404604457881959845e2_f64 * t825 * t326 * t43486;
    let t43746 = 0.43710935587469654631e2_f64 * t7584 * t7585 * t43598;
    let t43747 = 0.14300195980740170668e1_f64 * t43690 + t43693 - t43695 - t43698 + 0.14300195980740170668e1_f64 * t43699 + 0.35750489951850426669e0_f64 * t780 * t43703 + t43708 + 0.38342925953920749676e0_f64 * t43712 - t43716 + t43719 + t43721 + t43723 + t43726 + t43729 + 0.14300195980740170668e1_f64 * t43731 - t43735 + t43737 - t43740 - t43743 - t43746;
    t43747
}
