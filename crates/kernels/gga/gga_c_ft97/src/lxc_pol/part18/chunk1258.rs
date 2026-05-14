//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1258/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1258<F: Float>(t26272: F, t8392: F, t26350: F, t11442: F, t11859: F, t1643: F, t1651: F, t1901: F, t1909: F, t22862: F, t26267: F, t26390: F, t3193: F, t379: F, t446: F, t452: F, t5691: F, t8557: F, t91771: F, t92086: F, t92096: F, t93577: F, t93579: F, t93609: F, t93612: F, t93621: F, t986: F) -> (F,) {
    let t103695 = 2.0 / 27.0 * t8392 * t26272;
    let t103698 = 4.0 / 81.0 * t8392 * t26350;
    let t103723 = -t446 * t452 * t986 * t22862 / 3.0 - 2.0 / 81.0 * t92086 - t103695 - 2.0 / 9.0 * t92096 + t103698 - 2.0 / 9.0 * t1901 * t8557 * t26390 * t379 - t93577 / 9.0 + t1901 * t1909 * t26267 * t1651 / 9.0 + 2.0 / 27.0 * t1901 * t3193 * t26267 * t1643 - 2.0 / 9.0 * t93579 - t1901 * t8557 * t5691 * t11442 / 9.0 - 2.0 / 9.0 * t1901 * t91771 * t11859 - 8.0 / 27.0 * t93609 - 8.0 / 27.0 * t93612 - 8.0 / 27.0 * t93621;
    (t103723,)
}
