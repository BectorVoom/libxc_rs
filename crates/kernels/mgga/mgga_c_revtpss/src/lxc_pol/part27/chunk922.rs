//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 922/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk922<F: Float>(t12292: F, t12296: F, t12297: F, t12299: F, t12301: F, t12303: F, t12307: F, t12310: F, t12314: F, t12317: F, t12320: F, t1132: F, t409: F, t416: F, t1134: F, t3391: F) -> (F, F, F, F) {
    let t12322 = -t12296 + 4.0 / 9.0 * t12297 + 2.0 / 9.0 * t12299 - 2.0 / 3.0 * t12301 - t12303 / 3.0 + 10.0 / 27.0 * t12307 - 4.0 / 3.0 * t12310 - 2.0 / 3.0 * t12292 + 2.0 * t12314 + 2.0 * t12317 + t12320 / 3.0;
    let t12323 = t1132 * t12322;
    let t12327 = 1.0 / t409 / t416 / 4.0;
    let t12328 = t3391 * t1134;
    (t12322, t12323, t12327, t12328)
}
