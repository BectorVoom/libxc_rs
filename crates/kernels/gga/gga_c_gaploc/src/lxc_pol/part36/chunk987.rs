//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 987/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk987<F: Float>(t10086: F, t10811: F, t43508: F, t7427: F, t7573: F, t326: F, t43486: F, t825: F, t43598: F, t7584: F, t7585: F, t43690: F, t43693: F, t43695: F, t43698: F, t43699: F, t43703: F, t43708: F, t43712: F, t43716: F, t43719: F, t43721: F, t43723: F, t43726: F, t43729: F, t43731: F, t43735: F, t780: F) -> F {
    let t43737 = F::new(0.42900587942220512003e1) * t10811 * t10086;
    let t43740 = F::new(0.62115540045351614476e2) * t7427 * t7573 * t43508;
    let t43743 = F::new(0.18404604457881959845e2) * t825 * t326 * t43486;
    let t43746 = F::new(0.43710935587469654631e2) * t7584 * t7585 * t43598;
    let t43747 = F::new(0.14300195980740170668e1) * t43690 + t43693 - t43695 - t43698 + F::new(0.14300195980740170668e1) * t43699 + F::new(0.35750489951850426669e0) * t780 * t43703 + t43708 + F::new(0.38342925953920749676e0) * t43712 - t43716 + t43719 + t43721 + t43723 + t43726 + t43729 + F::new(0.14300195980740170668e1) * t43731 - t43735 + t43737 - t43740 - t43743 - t43746;
    t43747
}
