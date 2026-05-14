//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1096/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1096<F: Float>(t32885: F, t32887: F, t32891: F, t32893: F, t32897: F, t32899: F, t32901: F, t32905: F, t32910: F, t32913: F, t32917: F, t32921: F, t9649: F, t9652: F, t9664: F, t1333: F, t9691: F) -> (F, F) {
    let t32924 = -0.69444444444444444446e-2 * t32885 - 0.69444444444444444446e-2 * t32887 + t32891 + 0.40208333333333333335e-2 * t9649 * t32893 - t32897 + 0.24872916666666666666e-2 * t32899 + 0.22109259259259259258e-2 * t32901 + 0.33163888888888888888e-2 * t32905 + 0.10416666666666666667e-1 * t9664 * t32893 + 0.69444444444444444446e-2 * t32910 + 0.10416666666666666667e-1 * t9664 * t32913 + 0.69444444444444444446e-2 * t9664 * t32917 + 0.8041666666666666667e-2 * t32921 * t9652;
    let t32925 = t1333 * t9691;
    (t32924, t32925)
}
