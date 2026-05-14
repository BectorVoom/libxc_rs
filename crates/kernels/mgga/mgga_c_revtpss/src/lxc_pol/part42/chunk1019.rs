//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1019/1363 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1019<F: Float>(t1514: F, t2289: F, t4264: F, t625: F, t4288: F, t2349: F, t97: F, t105: F, t2357: F, t1857: F, t3857: F, t177: F, t5566: F, t762: F, t1450: F, t5778: F) -> (F, F, F, F, F, F, F, F) {
    let t13448 = t2289 * t1514;
    let t13451 = 4.0 / 3.0 * t625 * t4264;
    let t13453 = 2.0 / 3.0 * t625 * t4288;
    let t13475 = t97 * t2349;
    let t13496 = t105 * t2357;
    let t13584 = t3857 * t1857;
    let t13597 = t5566 * t177;
    let t13599 = 0.11696447245269292414e1 * t13597 * t762;
    let t13600 = t5778 * t1450;
    (t13448, t13451, t13453, t13475, t13496, t13584, t13599, t13600)
}
