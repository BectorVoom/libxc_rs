//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1118/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1118<F: Float>(t13776: F, t3212: F, t50956: F, t875: F, t51651: F, t13812: F, t2503: F, t13817: F, t14418: F, t859: F, t892: F, t1177: F, t1178: F, t371: F, t9689: F, t14617: F, t51581: F) -> (F, F, F, F, F, F, F) {
    let t53748 = t13776 * t50956 * t3212 * t875;
    let t53750 = 35.0 / 108.0 * t51651;
    let t53751 = t13812 * t2503;
    let t53758 = t13817 * t2503;
    let t53761 = t859 * t892 * t14418;
    let t53768 = t1177 * t371 * t1178 * t9689;
    let t53772 = t51581 * t14617;
    (t53748, t53750, t53751, t53758, t53761, t53768, t53772)
}
