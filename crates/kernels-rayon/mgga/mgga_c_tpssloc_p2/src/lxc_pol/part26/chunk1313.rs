//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1313/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1313(t23197: f64, t6547: f64, t23222: f64, t23237: f64, t6552: f64, t23257: f64, t6562: f64, t794: f64, t10111: f64, t1911: f64, t22975: f64, t23191: f64, t23215: f64, t2597: f64, t2713: f64, t2718: f64, t2742: f64, t40890: f64, t6662: f64, t82219: f64, t82221: f64, t82228: f64, t855: f64) -> f64 {
    let t82230 = t6547 * t23197;
    let t82233 = t6552 * t23237 * t23222;
    let t82236 = t6562 * t794 * t23257;
    let t82246 = 6.0_f64 * t855 * t2718 * t6662 * t2742 - t82219 + 0.49348022005446793095e-1_f64 * t82221 + 6.0_f64 * t2713 * t22975 - 0.14804406601634037928e0_f64 * t82228 - 0.11514538467937585055e0_f64 * t82230 - 0.49348022005446793095e-1_f64 * t82233 - 0.12337005501361698274e-1_f64 * t82236 - 18.0_f64 * t2597 * t23215 - 3.0_f64 * t2713 * t23191 + 24.0_f64 * t855 * t40890 * t1911 * t10111;
    t82246
}
