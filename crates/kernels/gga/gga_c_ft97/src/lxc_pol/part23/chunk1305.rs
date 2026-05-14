//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1305/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1305<F: Float>(t111592: F, t111807: F, t1253: F, t19867: F, t2347: F, t2360: F, t25412: F, t25413: F, t25459: F, t28934: F, t28935: F, t28938: F, t28941: F, t28944: F, t28947: F, t28951: F, t29006: F, t29008: F, t31646: F, t31657: F, t31658: F, t3886: F, t5408: F, t6216: F, t684: F, t98273: F, t98694: F) -> (F,) {
    let t125493 = 2.0 / 9.0 * t6216 * t28938 * t1253 * t2360 * t3886 - 2.0 / 27.0 * t6216 * t28944 * t1253 * t2347 * t3886 + 2.0 * t29008 * t29006 - t6216 * t98273 * t31646 * t684 / 3.0 + 2.0 / 9.0 * t6216 * t111807 * t28934 + 2.0 / 9.0 * t29008 * t28935 + 2.0 / 9.0 * t29008 * t28941 - 2.0 / 27.0 * t29008 * t28947 + 2.0 / 9.0 * t29008 * t28951 + 2.0 / 9.0 * t25459 * t31658 + 2.0 / 9.0 * t6216 * t98694 * t31657 + 2.0 / 9.0 * t6216 * t25412 * t111592 * t5408 + 2.0 / 9.0 * t6216 * t25412 * t25413 * t19867;
    (t125493,)
}
