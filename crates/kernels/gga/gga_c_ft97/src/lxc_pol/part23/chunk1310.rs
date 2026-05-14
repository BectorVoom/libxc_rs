//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1310/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1310<F: Float>(t1466: F, t31647: F, t681: F, t19430: F, t99918: F, t18497: F, t18514: F, t18712: F, t193: F, t19338: F, t19460: F, t25412: F, t25413: F, t28835: F, t28938: F, t28939: F, t28944: F, t28945: F, t28968: F, t28978: F, t29000: F, t29033: F, t29035: F, t5422: F, t6216: F, t6261: F, t6963: F) -> (F, F) {
    let t125634 = t1466 * t681 * t31647;
    let t125636 = t99918 * t19430;
    let t125652 = -2.0 / 9.0 * t6216 * t25412 * t28939 * t19460 - t6216 * t28938 * t28945 * t18514 / 3.0 - 4.0 / 9.0 * t29000 * t28938 * t28939 * t18497 + t6216 * t25412 * t25413 * t19338 / 9.0 + t6216 * t28938 * t28939 * t18712 / 9.0 - t6216 * t28944 * t28945 * t18712 / 27.0 - t125634 / 3.0 - 12.0 * t125636 - 2.0 / 3.0 * t1466 * t193 * t28835 * t29033 + t1466 * t193 * t6261 * t5422 / 6.0 - 2.0 / 3.0 * t6963 * t28968 - 2.0 / 3.0 * t6963 * t29035 - 2.0 / 3.0 * t6963 * t28978;
    (t125636, t125652)
}
