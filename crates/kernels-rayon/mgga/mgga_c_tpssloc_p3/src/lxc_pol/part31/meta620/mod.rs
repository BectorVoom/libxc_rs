//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta620 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1871;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1872;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1873;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta620(t22704: f64, t22705: f64, t28167: f64, t26331: f64, t26421: f64, t26446: f64, t5187: f64, t1992: f64, t22897: f64, t3792: f64, t57607: f64, t19745: f64, t81027: f64, t12369: f64, t19743: f64, t22633: f64, t562: f64, t6330: f64, t1307: f64, t90591: f64, t20018: f64, t6976: f64, t550: f64, t57499: f64, t28163: f64, t57618: f64, t22881: f64, t6347: f64, t6637: f64, t6888: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t96989, t96993, t96997, t97002) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1871(t22704, t22705, t28167, t26331, t26421, t26446, t5187, t1992, t22897, t3792, t57607, t19745, t81027);
        let (t97007, t97011, t97014, t97017) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1872(t12369, t19743, t22633, t22897, t562, t6330, t1307, t26446, t90591, t1992, t20018, t6976);
        let (t97023, t97026, t97030, t97036) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1873(t1992, t550, t57499, t6976, t22704, t22705, t28163, t57618, t22881, t6347, t6637, t6888);
    (t96989, t96993, t96997, t97002, t97007, t97011, t97014, t97017, t97023, t97026, t97030, t97036)
}
