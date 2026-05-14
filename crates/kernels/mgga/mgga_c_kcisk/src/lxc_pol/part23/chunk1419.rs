//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1419/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1419<F: Float>(t32401: F, t9860: F, t33854: F, t9532: F, t33830: F, t3936: F, t109390: F, t2326: F, t4348: F, t6204: F, t115078: F, t33937: F, t33928: F, t109626: F, t109683: F, t109690: F, t1299: F, t20: F, t21526: F, t21600: F, t2734: F, t2740: F, t32363: F, t32366: F, t32439: F, t33850: F, t33960: F, t394: F, t6448: F, t9511: F, t9855: F) -> (F, F) {
    let t115393 = 0.34722222222222222222e-2 * t9860 * t32401;
    let t115404 = 0.34722222222222222222e-2 * t33854 * t9532;
    let t115416 = t3936 * t33830;
    let t115423 = t6204 * t109390 * t2326 * t4348;
    let t115426 = t33937 * t115078;
    let t115430 = 0.34722222222222222222e-2 * t33928 * t9532;
    let t115431 = t115393 + 0.52083333333333333333e-2 * t32363 * t9855 + 0.10416666666666666667e-1 * t32366 * t9855 + 0.27777777777777777778e-1 * t2734 * t6448 * t1299 * t20 * t2740 - t115404 - 0.10416666666666666667e-1 * t9511 * t33960 * t2740 - 0.52083333333333333333e-2 * t2734 * t21600 * t394 * t20 * t2740 + 0.27777777777777777778e-1 * t9511 * t33850 * t2740 - 0.69444444444444444445e-2 * t109626 * t115416 * t21526 + 0.77160493827160493826e-3 * t109683 + 0.60312500000000000001e-2 * t32439 * t115423 - 0.77602083333333333334e-3 * t115426 - 0.17361111111111111111e-2 * t109690 - t115430;
    (t115423, t115431)
}
