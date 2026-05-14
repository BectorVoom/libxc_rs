//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1290/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1290<F: Float>(t111636: F, t111679: F, t111723: F, t111754: F, t111802: F, t112393: F, t112425: F, t112455: F, t112491: F, t112524: F, t112562: F, t112603: F, t112644: F, t113: F, t114998: F, t115028: F, t115046: F, t1275: F, t14391: F, t14403: F, t14409: F, t14563: F, t14582: F, t1512: F, t1577: F, t1934: F, t25500: F, t25504: F, t29425: F, t29429: F, t2966: F, t332: F, t4382: F, t5: F, t505: F, t6403: F, t7138: F, t98250: F, t992: F) -> (F,) {
    let t115067 = t6403 * t14403 / 2.0 + t25504 * t4382 / 2.0 + t6403 * t14391 / 4.0 + t6403 * t14409 / 2.0 + t5 * t25500 * t992 / 4.0 - t5 * t1512 * t1577 / 2.0 + t6403 * t14563 / 4.0 + t5 * (t111636 + t111679 + t111723 + t111754 + t111802 + t112393 + t112425 + t112455 + t112491 + t112524 + t112562 + t112603 + t112644 + t114998 + t115028 + t115046) * t332 * t113 / 4.0 + t6403 * t14582 / 4.0 + t5 * t7138 * t1934 / 4.0 + t98250 * t1275 / 4.0 + t29429 * t2966 / 2.0 + t5 * t29425 * t505 / 2.0;
    (t115067,)
}
