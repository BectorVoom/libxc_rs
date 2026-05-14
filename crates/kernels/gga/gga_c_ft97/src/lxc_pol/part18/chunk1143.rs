//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1143/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1143<F: Float>(t1284: F, t1900: F, t7149: F, t38477: F, t5502: F, t22914: F, t26119: F, t11064: F, t11520: F, t11525: F, t1286: F, t1920: F, t22922: F, t22935: F, t25525: F, t25530: F, t25570: F, t28: F, t3266: F, t5495: F, t5501: F, t5507: F, t6418: F, t6421: F, t6455: F, t8411: F, t93468: F, t93864: F, t94089: F, t942: F) -> (F, F) {
    let t100127 = t1284 * t7149 * t1900;
    let t100128 = t38477 * t5502;
    let t100147 = t22914 * t26119 / 27.0;
    let t100163 = 2.0 * t5501 * t8411 * t22922 * t3266 - 2.0 / 3.0 * t100127 * t100128 * t11064 - 8.0 / 27.0 * t93864 - 2.0 / 3.0 * t5495 * t25530 - 2.0 / 3.0 * t5495 * t25525 - t1286 * t28 * t93468 * t6421 / 3.0 - t1286 * t28 * t5507 * t1920 * t942 / 3.0 + t100147 + t1286 * t28 * t6455 * t1920 / 6.0 + 2.0 * t5501 * t8411 * t5502 * t11520 + t5501 * t8411 * t5502 * t11525 - t94089 * t6418 / 18.0 - t22935 * t25570 / 9.0;
    (t100127, t100163)
}
