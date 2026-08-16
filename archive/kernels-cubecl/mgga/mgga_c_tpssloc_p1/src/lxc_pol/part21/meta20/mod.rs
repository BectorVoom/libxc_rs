//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta20 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk154;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk155;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk156;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk157;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk158;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk159;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk160;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk161;
use chunk8::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk162;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta20<F: Float>(t371: F, t67: F, t246: F, t120: F, t61: F, t283: F, t339: F, t350: F, t370: F, t349: F, t362: F, t68: F, t353: F, t254: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t372 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk154::<F>(t371);
        let (t373, t374) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk155::<F>(t372, t67, t246);
        let t375 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk156::<F>(t120, t61);
        let t376 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk157::<F>(t283);
        let t378 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk158::<F>(t374, t375, t376);
        let t381 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk159::<F>(t339, t350, t370, t378);
        let (t382, t383) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk160::<F>(t349, t381, t362, t68);
        let t384 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk161::<F>(t381, t383);
        let (t386, t388) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk162::<F>(t353, t384, t254);
    (t372, t373, t374, t375, t376, t378, t381, t382, t383, t384, t386, t388)
}
