//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1139/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1139<F: Float>(t1328: F, t587: F, t9438: F, t9439: F, t2487: F, t9448: F, t4379: F, t9580: F, t21077: F, t901: F, t2372: F, t6625: F) -> (F, F, F, F, F) {
    let t30326 = t587 * t9438 * t9439 * t1328;
    let t30330 = t2487 * t9438 * t9448 * t1328;
    let t30339 = F::cast_from(0.11916829983950142223e0_f64) * t4379 * t9580;
    let t30354 = F::cast_from(0.59584149919750711116e-1_f64) * t21077 * t901;
    let t30356 = F::cast_from(0.17875244975925213335e0_f64) * t2372 * t6625;
    (t30326, t30330, t30339, t30354, t30356)
}
