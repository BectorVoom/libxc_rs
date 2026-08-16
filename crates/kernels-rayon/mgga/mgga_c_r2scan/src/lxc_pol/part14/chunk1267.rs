//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1267/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1267(t3275: f64, t39010: f64, t39318: f64, t11523: f64, t12052: f64, t12219: f64, t37282: f64, t11011: f64, t12056: f64, t3262: f64, t3465: f64, t40492: f64) -> (f64, f64, f64, f64, f64) {
    let t42302 = 585.0_f64 / 256.0_f64 * t3275 * t39010 * t39318;
    let t42304 = t11523 * t12052 / 2.0_f64;
    let t42307 = 15.0_f64 / 8.0_f64 * t37282 * t12219;
    let t42310 = 3.0_f64 / 2.0_f64 * t3262 * t12056 * t11011;
    let t42313 = 3.0_f64 / 2.0_f64 * t3262 * t3465 * t40492;
    (t42302, t42304, t42307, t42310, t42313)
}
