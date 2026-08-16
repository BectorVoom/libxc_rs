//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1131/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1131(t225: f64, t26221: f64, t26329: f64, t26229: f64, t2022: f64, t671: f64, t7450: f64, t1307: f64, t1842: f64, t1527: f64, t776: f64, t31253: f64, t580: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t91441 = t26221 * t225;
    let t91488 = t26329 * t225;
    let t91491 = t26229 * t225;
    let t96351 = t2022 * t671;
    let t96361 = t7450 * t671;
    let t97721 = t1842 * t1307;
    let t98960 = t1527 * t776;
    let t112516 = t31253 * t580;
    (t91441, t91488, t91491, t96351, t96361, t97721, t98960, t112516)
}
