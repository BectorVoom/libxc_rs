//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1257/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1257<F: Float>(t18325: F, t32947: F, t62249: F, t9664: F, t9671: F, t10798: F, t33031: F, t33060: F, t32989: F) -> (F, F, F, F) {
    let t112269 = t32947 * t18325;
    let t112283 = t9664 * t62249 * t9671;
    let t112286 = t33031 * t10798 * t33060;
    let t112289 = t32989 * t18325;
    (t112269, t112283, t112286, t112289)
}
