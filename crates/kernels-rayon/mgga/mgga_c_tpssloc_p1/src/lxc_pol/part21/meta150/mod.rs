//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta150 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk975;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk976;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk977;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta150(t3551: f64, t974: f64, t1176: f64, t3247: f64, t2244: f64, t3242: f64, t3439: f64, t225: f64, t3481: f64, t68: f64, t484: f64, t121: f64, t486: f64, t1216: f64, t248: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3552, t3555, t3556, t3557, t3560, t3561, t3562, t3565, t3566) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk975(t3551, t974, t1176, t3247, t2244, t3242, t3439, t225, t3481, t68);
        let (t3567, t3570) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk976(t3566, t484, t121, t486);
        let t3572 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk977(t1216, t248, t3570);
    (t3552, t3555, t3556, t3557, t3560, t3561, t3562, t3565, t3566, t3567, t3570, t3572)
}
