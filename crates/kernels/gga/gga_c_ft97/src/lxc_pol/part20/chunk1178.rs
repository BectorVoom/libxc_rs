//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1178/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1178<F: Float>(t1091: F, t111682: F, t111685: F, t111687: F, t111703: F, t111705: F, t111711: F, t111716: F, t1212: F, t1466: F, t1479: F, t193: F, t2: F, t26: F, t2665: F, t2892: F, t4: F, t4309: F, t6216: F, t6222: F, t6261: F, t684: F, t72995: F, t98278: F, t98281: F, t99993: F) -> (F,) {
    let t111723 = -4.0 / 27.0 * t111682 + t111685 - 11.0 / 27.0 * t111687 - t1466 * t193 * t6222 * t2892 * t1212 / 3.0 + t1466 * t193 * t6261 * t4309 / 3.0 + t72995 * t2 * t4 * t26 * t1479 / 6.0 - 4.0 * t111703 - 2.0 / 81.0 * t111705 - t6216 * t2665 * t99993 * t1091 / 18.0 - t6216 * t2665 * t111711 * t684 / 9.0 - t6216 * t2665 * t111716 * t684 / 9.0 - t98278 / 9.0 - t98281 / 18.0;
    (t111723,)
}
