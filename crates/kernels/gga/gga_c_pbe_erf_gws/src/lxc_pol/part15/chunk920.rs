//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 920/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk920<F: Float>(t2150: F, t9188: F, t3180: F, t6322: F, t3131: F, t6523: F, t6524: F, t2168: F, t9140: F, t9142: F, t9143: F, t9145: F, t9174: F, t9175: F, t9177: F, t9181: F, t9183: F, t9187: F) -> (F, F, F, F, F) {
    let t9190 = t9188 * t2150 / 24.0;
    let t9192 = t6322 * t3180 / 48.0;
    let t9194 = t6523 * t3131 * t6524;
    let t9196 = t2168 * t9194 / 16.0;
    let t9197 = t9140 - t9142 - t9143 - t9145 + t9174 + t9175 - t9177 + t9181 + t9183 + t9187 - t9190 - t9192 - t9196;
    (t9190, t9192, t9194, t9196, t9197)
}
