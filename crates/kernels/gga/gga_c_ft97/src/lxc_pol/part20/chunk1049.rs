//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1049/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1049<F: Float>(t6749: F, t98152: F, t1403: F, t27946: F, t681: F, t27893: F, t92: F, t13927: F, t24415: F, t24418: F, t14354: F, t1454: F, t193: F, t2347: F, t2354: F, t2360: F, t2409: F, t24231: F, t24242: F, t2617: F, t28015: F, t28018: F, t28030: F, t28036: F, t3875: F, t3886: F, t6002: F, t6005: F, t6838: F, t771: F, t96339: F, t96392: F) -> (F, F, F) {
    let t107832 = t98152 * t6749 / 27.0;
    let t107835 = t1403 * t681 * t27946 / 9.0;
    let t107836 = t27893 * t92;
    let t107845 = t13927 * t24415;
    let t107866 = t13927 * t24418;
    let t107869 = t107832 - t107835 - t107836 * t6005 / 9.0 - t28015 * t24242 / 9.0 + t6002 * t2354 * t28018 * t2409 / 9.0 + 4.0 * t107845 + t1403 * t193 * t6838 * t2617 / 6.0 + 2.0 / 9.0 * t6002 * t24231 * t96339 * t3875 + 2.0 / 9.0 * t6002 * t28030 * t771 * t2360 * t3886 - 2.0 / 27.0 * t6002 * t28036 * t771 * t2347 * t3886 - t14354 * t1454 + 8.0 * t107866 - t96392 / 18.0;
    (t107845, t107866, t107869)
}
