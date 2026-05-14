//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1160/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1160<F: Float>(t11153: F, t218: F, t219: F, t824: F, t31086: F, t334: F, t11205: F, t675: F, t11209: F, t3747: F, t836: F, t7996: F, t7999: F, t22230: F, t22290: F, t22293: F, t22296: F, t22336: F, t27358: F, t27361: F, t27370: F, t27373: F, t31240: F, t31242: F, t31250: F, t31254: F) -> (F, F, F, F, F, F, F) {
    let t31258 = t218 * t219 * t824 * t11153;
    let t31262 = t218 * t219 * t334 * t31086;
    let t31265 = t218 * t675 * t11205;
    let t31268 = t218 * t675 * t11209;
    let t31270 = t3747 * t836;
    let t31271 = t7996 * t31270;
    let t31273 = t7999 * t31270;
    let t31275 = -0.27903555555555555556e1 * t22230 + t22336 - 0.21908444444444444444e1 * t22290 + 0.82156666666666666666e0 * t22293 + 0.82156666666666666666e0 * t22296 + 0.1898925e1 * t31240 + 0.3071625e0 * t31242 + 0.82156666666666666665e0 * t27358 - 0.98587999999999999998e0 * t27361 - 0.49293999999999999999e0 * t27370 - 0.49293999999999999999e0 * t27373 + 0.73941e0 * t31250 + 0.73941e0 * t31254 + 0.24647e0 * t31258 + 0.24647e0 * t31262 - 0.49294e0 * t31265 - 0.16431333333333333333e0 * t31268 + 0.427258125e1 * t31271 - 0.230371875e0 * t31273;
    (t31258, t31262, t31265, t31268, t31271, t31273, t31275)
}
