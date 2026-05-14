//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1025/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1025<F: Float>(t1411: F, t3177: F, t587: F, t1328: F, t9438: F, t9439: F, t2487: F, t9448: F, t4379: F, t9580: F, t21077: F, t901: F, t2372: F, t6625: F, t2464: F, t2465: F, t6417: F) -> (F, F, F, F, F, F, F) {
    let t30323 = 0.11928910296775344344e1 * t587 * t1411 * t3177;
    let t30326 = t587 * t9438 * t9439 * t1328;
    let t30330 = t2487 * t9438 * t9448 * t1328;
    let t30339 = 0.11916829983950142223e0 * t4379 * t9580;
    let t30354 = 0.59584149919750711116e-1 * t21077 * t901;
    let t30356 = 0.17875244975925213335e0 * t2372 * t6625;
    let t30374 = 0.17041300423964777634e0 * t587 * t2464 * t2465 * t6417;
    (t30323, t30326, t30330, t30339, t30354, t30356, t30374)
}
