//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2566/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2566(t51246: f64, t5989: f64, t1098: f64, t21988: f64, t1119: f64, t50834: f64, t51257: f64, t63291: f64, t63306: f64, t63308: f64, t63841: f64, t63843: f64, t63845: f64, t71333: f64, t71335: f64, t71337: f64) -> (f64, f64, f64) {
    let t71876 = 6.0_f64 * t51246 * t5989;
    let t71877 = t21988 * t1098;
    let t71879 = 1.0_f64 * t71877 * t1119;
    let t71902 = -0.103295e1_f64 * t63291 + 0.34431666666666666666e0_f64 * t63306 - 0.5738611111111111111e0_f64 * t63308 - 0.34731666666666666667e-1_f64 * t71333 + 0.69463333333333333333e-1_f64 * t71335 - 0.41678e0_f64 * t71337 + t51257 - 0.16068111111111111111e1_f64 * t50834 - 0.9261777777777777778e-1_f64 * t63841 - 0.41678e0_f64 * t63843 + 0.69463333333333333333e-1_f64 * t63845;
    (t71876, t71879, t71902)
}
