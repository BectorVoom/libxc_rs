//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta668 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2469;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2470;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta668(t42341: f64, t44696: f64, t42344: f64, t483: f64, t1210: f64, t1174: f64, t3561: f64, t698: f64, t10471: f64, t44690: f64, t11727: f64, t44722: f64, t478: f64, t11818: f64, t1213: f64, t248: f64, t3494: f64, t3506: f64, t3509: f64, t3515: f64, t3516: f64, t11718: f64, t11721: f64, t3493: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t44833, t44834, t44836, t44847, t44857, t44858, t44863) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2469(t42341, t44696, t42344, t483, t1210, t1174, t3561, t698, t10471, t44690, t11727, t44722, t478);
        let (t44886, t44890, t44894, t44896, t44906) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2470(t11818, t1213, t248, t3494, t3506, t3509, t3515, t3516, t11718, t44857, t11721, t3493);
    (t44833, t44834, t44836, t44847, t44857, t44858, t44863, t44886, t44890, t44894, t44896, t44906)
}
