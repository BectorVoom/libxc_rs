//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1284/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1284(t31270: f64, t7996: f64, t7999: f64, t22230: f64, t22290: f64, t22293: f64, t22296: f64, t22336: f64, t27358: f64, t27361: f64, t27370: f64, t27373: f64, t31240: f64, t31242: f64, t31250: f64, t31254: f64, t31258: f64, t31262: f64, t31265: f64, t31268: f64) -> (f64, f64, f64) {
    let t31271 = t7996 * t31270;
    let t31273 = t7999 * t31270;
    let t31275 = -0.27903555555555555556e1_f64 * t22230 + t22336 - 0.21908444444444444444e1_f64 * t22290 + 0.82156666666666666666e0_f64 * t22293 + 0.82156666666666666666e0_f64 * t22296 + 0.1898925e1_f64 * t31240 + 0.3071625e0_f64 * t31242 + 0.82156666666666666665e0_f64 * t27358 - 0.98587999999999999998e0_f64 * t27361 - 0.49293999999999999999e0_f64 * t27370 - 0.49293999999999999999e0_f64 * t27373 + 0.73941e0_f64 * t31250 + 0.73941e0_f64 * t31254 + 0.24647e0_f64 * t31258 + 0.24647e0_f64 * t31262 - 0.49294e0_f64 * t31265 - 0.16431333333333333333e0_f64 * t31268 + 0.427258125e1_f64 * t31271 - 0.230371875e0_f64 * t31273;
    (t31271, t31273, t31275)
}
