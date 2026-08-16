//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 969/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk969(t2268: f64, t42212: f64, t888: f64, t39717: f64, t12800: f64, t6313: f64, t6305: f64, t2792: f64, t3158: f64, t12773: f64, t12810: f64, t10156: f64, t2349: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t42678 = t2268 * t42212 * t888;
    let t42687 = 0.47425011059460249332e-2_f64 * t39717;
    let t42689 = 0.26558006193297739625e0_f64 * t6313 * t12800;
    let t42691 = 0.19918504644973304719e0_f64 * t6305 * t12800;
    let t42694 = 0.19918504644973304719e0_f64 * t2268 * t3158 * t2792;
    let t42695 = t6313 * t12773;
    let t42698 = 0.37940008847568199465e-1_f64 * t6313 * t12810;
    let t42700 = t2268 * t10156 * t2349;
    (t42678, t42687, t42689, t42691, t42694, t42695, t42698, t42700)
}
