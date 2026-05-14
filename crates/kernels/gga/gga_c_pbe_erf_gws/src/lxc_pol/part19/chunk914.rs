//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 914/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk914<F: Float>(t10185: F, t10217: F, t11285: F, t11306: F, t312: F, t10015: F, t10017: F, t10018: F, t10019: F, t10022: F, t10026: F, t10248: F, t10249: F, t10250: F, t4602: F, t4652: F, t4664: F, t4744: F, t4751: F, t4784: F, t4790: F, t6076: F, t7994: F) -> (F,) {
    let t11308 = t10185 + t10217 + t11285 + t11306;
    let t11309 = t11308 * t312;
    let t11310 = t10015 + t10017 + t4602 + t4744 + t4751 + t4652 - t7994 + t10018 + t4664 - t6076 + t10019 - t10022 - t10026 - t11309 - t4784 - t10248 - t4790 - t10249 + t10250;
    (t11310,)
}
