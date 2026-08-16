//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1114/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1114<F: Float>(t11883: F, t4219: F, t11902: F, t4223: F, t11906: F, t11869: F, t4241: F, t9561: F, t3067: F, t1114: F, t4056: F, t3068: F) -> (F, F, F, F, F, F) {
    let t12301 = t4219 * t11883;
    let t12304 = t4223 * t11902;
    let t12307 = t4223 * t11906;
    let t12310 = t4223 * t11869;
    let t12317 = t9561 * t4241;
    let t12319 = t3067 * t12317 / F::cast_from(3456.0_f64);
    let t12320 = t4056 * t1114;
    let t12321 = t3068 * t12320;
    (t12301, t12304, t12307, t12310, t12319, t12321)
}
