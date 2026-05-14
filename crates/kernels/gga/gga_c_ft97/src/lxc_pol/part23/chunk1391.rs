//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1391/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1391<F: Float>(t1882: F, t31779: F, t31766: F, t10443: F, t113869: F, t114164: F, t114194: F, t114196: F, t114197: F, t125574: F, t125684: F, t127919: F, t1476: F, t1901: F, t19373: F, t19404: F, t19430: F, t19886: F, t24898: F, t29071: F, t29128: F, t296: F, t31739: F, t446: F, t6353: F, t840: F, t99140: F) -> (F,) {
    let t127923 = t1882 * t31779;
    let t127944 = t1882 * t31766;
    let t127946 = -t114164 + 8.0 * t1901 * t29128 * t113869 * t19430 - t99140 - t446 * t296 * t125684 / 3.0 - t446 * t296 * t127919 / 3.0 - 2.0 / 9.0 * t127923 + t446 * t840 * t6353 * t19373 / 3.0 + 2.0 * t1901 * t29071 * t24898 * t19404 - t114194 + t1901 * t10443 * t31739 / 9.0 - t114196 + 8.0 / 27.0 * t114197 + 4.0 / 3.0 * t446 * t296 * t125574 - t446 * t840 * t19886 * t1476 / 3.0 + t127944 / 9.0;
    (t127946,)
}
