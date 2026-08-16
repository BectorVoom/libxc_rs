//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1502/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1502<F: Float>(t80019: F, t80047: F, t6414: F, t550: F, t3792: F, t39249: F, t39256: F, t39261: F, t39266: F, t39304: F, t39309: F, t79834: F, t79835: F, t79836: F, t79837: F, t79853: F, t79854: F) -> (F, F, F, F) {
    let t80048 = t80019 + t80047;
    let t80075 = t6414 * t6414;
    let t80076 = t80075 * t550;
    let t80085 = t80075 * t3792;
    let t80101 = -t79834 - t79835 - t79836 - t79837 - t39249 - t39256 - t79853 - t79854 - t39261 - t39266 - t39304 - t39309;
    (t80048, t80076, t80085, t80101)
}
