//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3230/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3230<F: Float>(t1248: F, t12717: F, t12751: F, t1287: F, t16695: F, t16771: F, t16775: F, t17454: F, t17818: F, t17864: F, t17876: F, t17880: F, t354: F, t45654: F, t45683: F, t45715: F, t45796: F, t45859: F, t45863: F, t5351: F, t56825: F, t56830: F, t58793: F, t58798: F, t58804: F, t59650: F, t59824: F) -> F {
    let t59916 = -F::cast_from(0.19756347548806534796e1_f64) * t17864 * t17876 - F::cast_from(0.19756347548806534796e1_f64) * t17880 * t17876 + F::cast_from(0.79025390195226139182e1_f64) * t12717 * t16771 * t1248 * t1287 + F::cast_from(0.39512695097613069591e1_f64) * t12717 * t16775 * t1248 * t1287 + F::cast_from(0.39512695097613069591e1_f64) * t12717 * t5351 * t354 * t58798 + F::cast_from(0.79025390195226139182e1_f64) * t45859 * t16695 * t58804 - F::cast_from(0.39512695097613069591e1_f64) * t45863 * t16695 * t58793 - F::cast_from(0.79025390195226139182e1_f64) * t45715 * t17818 - F::cast_from(0.79025390195226139182e1_f64) * t45683 * t17818 - F::cast_from(0.79025390195226139182e1_f64) * t12751 * t59824 * t17454 - F::cast_from(0.39512695097613069591e1_f64) * t12751 * t16695 * t56825 - F::cast_from(0.39512695097613069591e1_f64) * t12751 * t16695 * t45796 - F::cast_from(0.11853808529283920877e2_f64) * t45654 * t59650 * t56830;
    t59916
}
