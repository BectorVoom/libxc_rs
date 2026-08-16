//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2220/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2220(t1933: f64, t23479: f64, t88405: f64, t1409: f64, t1937: f64, t6722: f64, t1015: f64, t10475: f64, t13762: f64, t14041: f64, t1615: f64, t23419: f64, t23678: f64, t25652: f64, t25653: f64, t25658: f64, t25660: f64, t25661: f64, t3040: f64, t3120: f64, t360: f64, t4575: f64, t4579: f64, t4649: f64, t82516: f64, t82542: f64, t82754: f64, t83008: f64, t83134: f64, t88537: f64, t88655: f64) -> f64 {
    let t88689 = 0.20186378047070195428e-3_f64 * t1933 * t88405 * t23479;
    let t88692 = 0.16149102437656156342e-2_f64 * t6722 * t1409 * t1937;
    let t88702 = 0.20186378047070195428e-3_f64 * t25652 * t25653 * t23678 * t3120 + 0.60559134141210586284e-3_f64 * t88537 * t10475 * t1615 * t82516 * t3040 - 0.60559134141210586284e-3_f64 * t88537 * t25653 * t82542 * t3040 - 0.20186378047070195428e-3_f64 * t88655 * t25661 - 0.20186378047070195428e-3_f64 * t25652 * t1015 * t4649 * t25660 - 0.10093189023535097714e-3_f64 * t25652 * t25658 * t82754 * t360 - t88689 - t88692 + t83008 * t4579 / 1152.0_f64 + t23419 * t13762 / 1152.0_f64 + t23419 * t14041 / 2304.0_f64 + 0.16149102437656156342e-2_f64 * t83134 + t83008 * t4575 / 1152.0_f64;
    t88702
}
