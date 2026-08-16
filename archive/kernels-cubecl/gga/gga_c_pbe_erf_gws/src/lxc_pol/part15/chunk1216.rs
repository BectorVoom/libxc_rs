//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1216/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1216<F: Float>(t321: F, t50825: F, t1167: F, t2423: F, t3324: F, t810: F, t1172: F, t1198: F, t319: F, t13763: F, t8546: F, t2494: F, t944: F) -> (F, F, F, F, F, F) {
    let t52061 = t321 * t50825;
    let t52763 = t1167 * t2423;
    let t52767 = t3324 * t810;
    let t52774 = t1172 * t319 * t1198;
    let t52775 = t8546 * t13763;
    let t52782 = t2494 * t944;
    (t52061, t52763, t52767, t52774, t52775, t52782)
}
