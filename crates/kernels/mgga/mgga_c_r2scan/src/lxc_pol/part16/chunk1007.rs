//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1007/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1007<F: Float>(t12424: F, t12431: F, t12704: F, t39169: F, t39170: F, t39171: F, t39172: F, t39173: F, t39174: F, t39175: F, t39176: F, t39177: F, t41104: F, t41105: F, t41106: F, t41107: F) -> (F,) {
    let t42379 = t12424 + t39169 + t39170 + t39171 - t39172 + t39173 - t39174 - t39175 + t12704 + t39176 + t39177 + t41104 + t41105 + t41106 - t12431 - t41107;
    (t42379,)
}
