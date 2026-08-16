//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2178/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2178<F: Float>(t27303: F, t786: F, t789: F, t25296: F, t27216: F, t25407: F, t27353: F, t27357: F, t51529: F, t7775: F, t7779: F, t93126: F, t93158: F, t93161: F, t93167: F, t93175: F, t93177: F, t99202: F, t99206: F, t99212: F, t99216: F) -> F {
    let t99219 = F::cast_from(0.19514881078765566038e-1_f64) * t786 * t27303 * t789;
    let t99222 = F::cast_from(0.25702851531048074406e-1_f64) * t27216 * t25296;
    let t99227 = -F::cast_from(0.17347256376410398924e1_f64) * t27353 * t27357 * t51529 + F::cast_from(0.34270468708064099208e-2_f64) * t93158 - F::cast_from(0.45699670022203476294e-2_f64) * t93161 + F::cast_from(0.17135234354032049604e-1_f64) * t99202 - F::cast_from(0.24093411633903331839e-3_f64) * t99206 + F::cast_from(0.4336814094102599731e0_f64) * t93126 * t7775 - F::cast_from(0.3427046870806409921e-2_f64) * t99212 + t99216 + t99219 - F::cast_from(0.12851425765524037203e-1_f64) * t93167 - t99222 - F::cast_from(0.34270468708064099208e-2_f64) * t93175 - F::cast_from(0.4336814094102599731e0_f64) * t25407 * t7779 - F::cast_from(0.45699670022203476294e-2_f64) * t93177;
    t99227
}
