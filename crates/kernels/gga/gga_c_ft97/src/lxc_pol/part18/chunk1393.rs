//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1393/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1393<F: Float>(t5773: F, t9132: F, t40465: F, t104477: F, t106811: F, t107227: F, t107471: F, t107675: F, t12338: F, t12590: F, t12597: F, t13228: F, t1349: F, t1360: F, t1389: F, t2228: F, t23925: F, t24059: F, t24135: F, t26533: F, t26535: F, t26791: F, t26817: F, t28: F, t5766: F, t6616: F, t95418: F) -> (F,) {
    let t107703 = t9132 * t5773;
    let t107707 = t40465 * t5773;
    let t107734 = 8.0 * t107471 + 8.0 * t106811 - 2.0 * t107227 - 4.0 / 9.0 * t104477 * t107703 * t12338 + 4.0 / 27.0 * t104477 * t107707 * t12590 - t95418 / 18.0 + t1349 * t28 * t1360 * t13228 / 6.0 + t1349 * t28 * t6616 * t2228 / 6.0 - t26817 * t24135 / 18.0 - 2.0 / 3.0 * t1349 * t28 * t23925 * t26533 - t12597 * t1389 - 12.0 * t107675 - 2.0 / 3.0 * t5766 * t26535 - 2.0 / 3.0 * t1349 * t28 * t26791 * t24059;
    (t107734,)
}
