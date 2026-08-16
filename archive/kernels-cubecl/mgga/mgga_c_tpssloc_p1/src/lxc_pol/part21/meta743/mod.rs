//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta743 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2610;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2611;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta743<F: Float>(t11702: F, t5019: F, t1734: F, t3493: F, t11697: F, t15458: F, t3577: F, t15462: F, t44951: F, t4949: F, t1215: F, t5011: F, t1222: F, t15765: F, t3242: F, t3448: F, t11728: F, t13969: F, t15630: F, t11718: F, t52835: F, t11797: F, t5024: F, t11147: F, t15394: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t53142, t53149, t53155, t53158, t53161, t53176) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2610::<F>(t11702, t5019, t1734, t3493, t11697, t15458, t3577, t15462, t44951, t4949, t1215, t5011);
        let (t53185, t53187, t53220, t53238, t53246, t53249) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2611::<F>(t1222, t15765, t3242, t3448, t11728, t13969, t15630, t11718, t52835, t11797, t5024, t11147, t15394);
    (t53142, t53149, t53155, t53158, t53161, t53176, t53185, t53187, t53220, t53238, t53246, t53249)
}
