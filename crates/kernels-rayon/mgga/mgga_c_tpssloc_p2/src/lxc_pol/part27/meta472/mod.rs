//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta472 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1835;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1836;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1837;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta472(t23477: f64, t23479: f64, t6721: f64, t6739: f64, t6741: f64, t1937: f64, t23447: f64, t23449: f64, t23454: f64, t23457: f64, t23460: f64, t23463: f64, t23465: f64, t23469: f64, t23474: f64, t350: f64, t378: f64, t6747: f64, t344: f64, t6729: f64, t6740: f64, t3008: f64, t343: f64, t6734: f64, t3103: f64, t6755: f64, t3120: f64, t360: f64, t68: f64, t6744: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t23480, t23482) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1835(t23477, t23479, t6721, t6739);
        let (t23483, t23486) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1836(t23482, t6741, t1937, t23447, t23449, t23454, t23457, t23460, t23463, t23465, t23469, t23474, t23480, t350, t378, t6747);
        let (t23488, t23489, t23494, t23495, t23500, t23503, t23504) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1837(t344, t6729, t6740, t3008, t343, t6734, t3103, t6755, t3120, t360, t68, t6744);
    (t23480, t23482, t23483, t23486, t23488, t23489, t23494, t23495, t23500, t23503, t23504)
}
