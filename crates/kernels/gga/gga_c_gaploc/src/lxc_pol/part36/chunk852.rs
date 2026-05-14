//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 852/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk852<F: Float>(t43690: F, t43693: F, t43695: F, t43698: F, t43699: F, t43703: F, t43708: F, t43712: F, t43716: F, t43719: F, t43721: F, t43723: F, t43726: F, t43729: F, t43731: F, t43735: F, t43737: F, t43740: F, t43743: F, t43746: F, t780: F) -> (F,) {
    let t43747 = 0.14300195980740170668e1 * t43690 + t43693 - t43695 - t43698 + 0.14300195980740170668e1 * t43699 + 0.35750489951850426669e0 * t780 * t43703 + t43708 + 0.38342925953920749676e0 * t43712 - t43716 + t43719 + t43721 + t43723 + t43726 + t43729 + 0.14300195980740170668e1 * t43731 - t43735 + t43737 - t43740 - t43743 - t43746;
    (t43747,)
}
