//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1295/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1295<F: Float>(t1403: F, t30914: F, t681: F, t10157: F, t1091: F, t109634: F, t109643: F, t109670: F, t109735: F, t124337: F, t1427: F, t18641: F, t193: F, t2: F, t2354: F, t24204: F, t24240: F, t26: F, t28010: F, t28018: F, t30875: F, t3746: F, t4: F, t4003: F, t4973: F, t5059: F, t6002: F, t6003: F, t6192: F, t6838: F, t81994: F) -> (F,) {
    let t125164 = t1403 * t681 * t30914;
    let t125178 = t81994 * t2 * t4 * t26 * t1427 / 6.0 - t5059 * t6192 + t1403 * t193 * t6838 * t4003 / 3.0 - t6002 * t2354 * t109735 * t1091 / 9.0 - t24204 * t30875 / 27.0 + 2.0 / 9.0 * t125164 - t109634 - t109643 - t6002 * t2354 * t24240 * t4973 / 18.0 - 2.0 * t124337 + t109670 + t6002 * t10157 * t6003 * t18641 + 2.0 / 9.0 * t28010 * t2354 * t28018 * t3746;
    (t125178,)
}
