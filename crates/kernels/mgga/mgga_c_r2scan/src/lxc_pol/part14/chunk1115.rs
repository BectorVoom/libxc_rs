//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1115/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1115<F: Float>(t3275: F, t39010: F, t39318: F, t11523: F, t12052: F, t12219: F, t37282: F, t11011: F, t12056: F, t3262: F, t3465: F, t40492: F, t10610: F, t3472: F, t40487: F, t1115: F, t2526: F, t3270: F) -> (F, F, F, F, F, F, F) {
    let t42302 = 585.0 / 256.0 * t3275 * t39010 * t39318;
    let t42304 = t11523 * t12052 / 2.0;
    let t42307 = 15.0 / 8.0 * t37282 * t12219;
    let t42310 = 3.0 / 2.0 * t3262 * t12056 * t11011;
    let t42313 = 3.0 / 2.0 * t3262 * t3465 * t40492;
    let t42316 = 15.0 / 8.0 * t10610 * t3472 * t40487;
    let t42318 = t3270 * t1115 * t2526;
    (t42302, t42304, t42307, t42310, t42313, t42316, t42318)
}
