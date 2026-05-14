//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 759/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk759<F: Float>(t8744: F, t7465: F, t7466: F, t7469: F, t7479: F, t7481: F, t7485: F, t7489: F, t7497: F, t7500: F, t8184: F, t8185: F, t8740: F, t8748: F, t9277: F, t7529: F, t7531: F, t7551: F, t7572: F, t7574: F, t7590: F, t7607: F, t8190: F, t8192: F, t8193: F, t8195: F, t8205: F, t8209: F, t8754: F, t8756: F) -> (F, F) {
    let t9278 = 0.305625e-1 * t8744;
    let t9280 = t7465 - 0.56606566121287473723e-2 * t7466 + t7469 + 0.1048269742986805069e-2 * t7479 - 0.62896184579208304138e-3 * t7481 + t7485 + t7489 - t7497 + 0.62896184579208304138e-3 * t7500 + 0.62896184579208304138e-3 * t8740 + t9277 + t9278 + t8184 - t8185 - 0.7640625e-2 * t8748;
    let t9289 = -t8754 / 24.0 - t8756 / 24.0 - 0.41930789719472202758e-3 * t7529 + 0.94344276868812456207e-3 * t7531 + t8190 + t8192 + t8193 - 0.94344276868812456205e-2 * t7551 - t8195 + t7572 + t7574 - t7590 - t8205 - t7607 + t8209;
    (t9280, t9289)
}
