//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2100/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2100<F: Float>(t10046: F, t814: F, t225: F, t9520: F, t10647: F, t892: F, t2784: F, t2841: F, t22715: F, t268: F, t271: F) -> (F, F, F, F, F) {
    let t41520 = t814 * t10046;
    let t41554 = t9520 * t225;
    let t41618 = t10647 * t892;
    let t41623 = t2784 * t2841;
    let t41654 = t268 * t22715 * t271;
    (t41520, t41554, t41618, t41623, t41654)
}
