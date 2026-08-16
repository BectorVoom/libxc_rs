//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3073/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3073(t14845: f64, t4782: f64, t14914: f64, t4740: f64, t44159: f64, t5989: f64, t11180: f64, t6021: f64, t18835: f64, t3259: f64, t50826: f64, t50828: f64, t50834: f64, t63291: f64, t63296: f64, t63300: f64, t63304: f64, t63306: f64, t63308: f64, t63313: f64, t63317: f64, t63323: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t63763 = 4.0_f64 * t14845 * t4782;
    let t63765 = 2.0_f64 * t4740 * t14914;
    let t63767 = 2.0_f64 * t44159 * t5989;
    let t63769 = 1.0_f64 * t11180 * t6021;
    let t63771 = 2.0_f64 * t3259 * t18835;
    let t63784 = -0.23744444444444444444e-1_f64 * t63291 + 0.71233333333333333332e-1_f64 * t63296 + 0.35616666666666666666e-1_f64 * t63300 + 0.10685e0_f64 * t63304 + 0.79148148148148148146e-2_f64 * t63306 - 0.13191358024691358024e-1_f64 * t63308 - 0.23744444444444444444e-1_f64 * t63313 - 0.11872222222222222222e-1_f64 * t63317 + 0.31659259259259259258e-1_f64 * t50826 - 0.11872222222222222222e-1_f64 * t50828 - 0.36935802469135802468e-1_f64 * t50834 + 0.79148148148148148147e-1_f64 * t63323;
    (t63763, t63765, t63767, t63769, t63771, t63784)
}
