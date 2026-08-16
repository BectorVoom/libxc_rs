//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2250/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2250(t25836: f64, t3216: f64, t11094: f64, t7627: f64, t1068: f64, t1070: f64, t14662: f64, t1637: f64, t193: f64, t23738: f64, t23742: f64, t25840: f64, t25845: f64, t3209: f64, t3213: f64, t336: f64, t4696: f64, t4700: f64, t60941: f64, t6822: f64, t83468: f64, t83472: f64, t83479: f64, t88054: f64, t88097: f64, t88137: f64, t88179: f64, t88213: f64, t88742: f64, t88779: f64, t88827: f64, t88867: f64, t88900: f64, t88940: f64, t89556: f64, t89590: f64, t89623: f64, t89658: f64, t89690: f64) -> f64 {
    let t89698 = t25836 * t3216;
    let t89702 = t7627 * t11094;
    let t89729 = t193 * t336 * (t88054 + t88097 + t88137 + t88179 + t88213 + t88742 + t88779 + t88827 + t88867 + t88900 + t88940 + t89556 + t89590 + t89623 + t89658 + t89690) * t1070 - 2.0_f64 * t4700 * t89698 * t1068 + 2.0_f64 * t4700 * t89702 * t3213 - t4700 * t25840 * t3209 - t4700 * t83468 * t1637 + 4.0_f64 * t4700 * t83472 * t25845 - 2.0_f64 * t4700 * t23738 * t4696 - 6.0_f64 * t4700 * t83479 * t1637 * t3213 + 4.0_f64 * t4700 * t23742 * t60941 + 2.0_f64 * t4700 * t23742 * t1637 * t3209 - t4700 * t6822 * t14662;
    t89729
}
