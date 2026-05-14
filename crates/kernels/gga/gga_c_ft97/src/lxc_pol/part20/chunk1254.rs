//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1254/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1254<F: Float>(t29147: F, t8392: F, t10443: F, t112579: F, t113170: F, t113531: F, t11593: F, t1248: F, t15184: F, t15408: F, t15534: F, t1901: F, t24886: F, t24890: F, t24908: F, t25135: F, t2749: F, t2862: F, t2874: F, t2894: F, t29260: F, t29307: F, t296: F, t319: F, t3746: F, t446: F, t7036: F, t840: F, t871: F, t98880: F, t98904: F, t98906: F) -> (F,) {
    let t113665 = 2.0 / 27.0 * t8392 * t29147;
    let t113709 = -t113665 - 2.0 / 9.0 * t1901 * t24886 * t15184 - 4.0 / 9.0 * t11593 * t2874 * t24908 * t3746 + 4.0 / 3.0 * t446 * t296 * t112579 - t98880 / 27.0 - t98904 / 27.0 - 2.0 / 81.0 * t98906 + 2.0 / 3.0 * t446 * t2862 * t2894 * t7036 + 2.0 / 3.0 * t446 * t840 * t2749 * t29307 + t446 * t840 * t871 * t25135 * t1248 / 3.0 + 4.0 / 3.0 * t446 * t2862 * t319 * t113170 + 2.0 / 3.0 * t446 * t2862 * t319 * t113531 + t1901 * t24890 * t15534 / 9.0 - 4.0 / 9.0 * t11593 * t24890 * t15408 + 2.0 / 9.0 * t1901 * t10443 * t29260;
    (t113709,)
}
