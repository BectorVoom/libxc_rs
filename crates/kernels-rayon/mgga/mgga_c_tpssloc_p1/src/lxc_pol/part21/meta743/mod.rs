//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta743 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2610;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2611;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta743(t11702: f64, t5019: f64, t1734: f64, t3493: f64, t11697: f64, t15458: f64, t3577: f64, t15462: f64, t44951: f64, t4949: f64, t1215: f64, t5011: f64, t1222: f64, t15765: f64, t3242: f64, t3448: f64, t11728: f64, t13969: f64, t15630: f64, t11718: f64, t52835: f64, t11797: f64, t5024: f64, t11147: f64, t15394: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t53142, t53149, t53155, t53158, t53161, t53176) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2610(t11702, t5019, t1734, t3493, t11697, t15458, t3577, t15462, t44951, t4949, t1215, t5011);
        let (t53185, t53187, t53220, t53238, t53246, t53249) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2611(t1222, t15765, t3242, t3448, t11728, t13969, t15630, t11718, t52835, t11797, t5024, t11147, t15394);
    (t53142, t53149, t53155, t53158, t53161, t53176, t53185, t53187, t53220, t53238, t53246, t53249)
}
