//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1108/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1108<F: Float>(t1333: F, t9684: F, t2785: F, t32938: F, t32975: F, t32978: F, t32982: F, t32984: F, t32987: F, t32990: F, t32996: F, t32999: F, t33002: F, t33005: F, t9649: F, t9652: F, t9667: F, t9672: F) -> (F, F) {
    let t33008 = t1333 * t9684;
    let t33014 = -0.24872916666666666666e-2 * t32975 - 0.55273148148148148147e-3 * t32978 + 0.49745833333333333332e-2 * t32982 - 0.20833333333333333334e-1 * t32984 * t2785 - 0.33163888888888888888e-2 * t32987 + 0.20833333333333333334e-1 * t32990 * t9672 + 0.20833333333333333334e-1 * t32990 * t9652 + 0.69444444444444444446e-2 * t32996 + 0.22109259259259259258e-2 * t32999 - 0.23280625000000000001e-2 * t33002 * t33005 + 0.33163888888888888888e-2 * t33008 - 0.69444444444444444446e-2 * t32990 * t9667 - 0.8041666666666666667e-2 * t9649 * t32938;
    (t33008, t33014)
}
