//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1371/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1371<F: Float>(t27022: F, t8392: F, t2178: F, t5842: F, t12650: F, t12680: F, t12968: F, t13140: F, t13230: F, t1359: F, t1391: F, t1643: F, t1651: F, t1901: F, t1986: F, t2075: F, t2185: F, t2221: F, t23416: F, t23571: F, t27006: F, t3281: F, t3483: F, t358: F, t41198: F, t446: F, t569: F, t574: F, t605: F, t6626: F, t6718: F, t6725: F, t9115: F, t95659: F, t95676: F, t95707: F, t95714: F) -> (F,) {
    let t106759 = 2.0 / 27.0 * t8392 * t27022;
    let t106761 = t2178 * t5842;
    let t106795 = 2.0 / 27.0 * t95659 + t1901 * t2221 * t27006 * t1651 / 9.0 + 2.0 / 27.0 * t1901 * t9115 * t27006 * t1643 - t106759 + 8.0 / 81.0 * t95676 - 4.0 / 3.0 * t1901 * t13140 * t106761 * t3483 + t1901 * t41198 * t6626 / 9.0 - 4.0 / 3.0 * t1901 * t12968 * t23571 * t12650 - t446 * t574 * t13230 * t1359 / 3.0 + 2.0 / 3.0 * t446 * t2185 * t6725 * t1986 + 4.0 / 27.0 * t95707 + 4.0 / 27.0 * t95714 - 2.0 / 9.0 * t1901 * t12680 * t23416 - 2.0 / 9.0 * t3281 * t569 * t1391 * t358 + t446 * t574 * t605 * t6718 * t2075 / 3.0;
    (t106795,)
}
