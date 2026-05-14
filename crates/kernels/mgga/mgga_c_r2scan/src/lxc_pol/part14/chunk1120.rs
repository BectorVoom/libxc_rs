//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1120/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1120<F: Float>(t1353: F, t2337: F, t3781: F, t3787: F, t42248: F, t42253: F, t42255: F, t42257: F, t42260: F, t42265: F, t42267: F, t42270: F, t42277: F, t42281: F, t42284: F, t42287: F, t42290: F, t42292: F) -> (F,) {
    let t42360 = t1353 * t3787 + t2337 * t3781 - t42248 + t42253 - t42255 + t42257 + t42260 - t42265 - t42267 + t42270 - t42277 + t42281 - t42284 + t42287 - t42290 + t42292;
    (t42360,)
}
