//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1238/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1238<F: Float>(t102751: F, t102753: F, t102759: F, t102760: F, t102767: F, t102772: F, t102773: F, t102776: F, t102783: F, t11492: F, t11525: F, t11854: F, t12047: F, t1339: F, t1871: F, t1901: F, t1909: F, t23323: F, t3113: F, t3114: F, t3204: F, t379: F, t446: F, t8557: F, t91605: F, t91614: F, t93636: F) -> (F,) {
    let t102792 = -t102751 - 2.0 / 9.0 * t91605 + 2.0 / 9.0 * t1901 * t1909 * t102753 * t379 - t102759 - 4.0 / 81.0 * t102760 + 2.0 / 3.0 * t446 * t1871 * t1339 * t11525 - t102767 - 2.0 / 9.0 * t1901 * t23323 * t12047 - t102772 + 4.0 / 81.0 * t102773 - 2.0 / 27.0 * t91614 - 4.0 / 3.0 * t1901 * t102776 * t11492 + 2.0 / 9.0 * t1901 * t93636 * t3114 - 2.0 / 9.0 * t1901 * t8557 * t102783 * t3204 - 4.0 / 9.0 * t1901 * t11854 * t102783 * t3113;
    (t102792,)
}
