//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3230/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3230(t1248: f64, t12717: f64, t12751: f64, t1287: f64, t16695: f64, t16771: f64, t16775: f64, t17454: f64, t17818: f64, t17864: f64, t17876: f64, t17880: f64, t354: f64, t45654: f64, t45683: f64, t45715: f64, t45796: f64, t45859: f64, t45863: f64, t5351: f64, t56825: f64, t56830: f64, t58793: f64, t58798: f64, t58804: f64, t59650: f64, t59824: f64) -> f64 {
    let t59916 = -0.19756347548806534796e1_f64 * t17864 * t17876 - 0.19756347548806534796e1_f64 * t17880 * t17876 + 0.79025390195226139182e1_f64 * t12717 * t16771 * t1248 * t1287 + 0.39512695097613069591e1_f64 * t12717 * t16775 * t1248 * t1287 + 0.39512695097613069591e1_f64 * t12717 * t5351 * t354 * t58798 + 0.79025390195226139182e1_f64 * t45859 * t16695 * t58804 - 0.39512695097613069591e1_f64 * t45863 * t16695 * t58793 - 0.79025390195226139182e1_f64 * t45715 * t17818 - 0.79025390195226139182e1_f64 * t45683 * t17818 - 0.79025390195226139182e1_f64 * t12751 * t59824 * t17454 - 0.39512695097613069591e1_f64 * t12751 * t16695 * t56825 - 0.39512695097613069591e1_f64 * t12751 * t16695 * t45796 - 0.11853808529283920877e2_f64 * t45654 * t59650 * t56830;
    t59916
}
