//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2721/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2721(t1345: f64, t1347: f64, t1348: f64, t16148: f64, t16176: f64, t16186: f64, t16191: f64, t16202: f64, t1819: f64, t1821: f64, t19702: f64, t19725: f64, t19728: f64, t1995: f64, t3734: f64, t3839: f64, t3843: f64, t3847: f64, t5272: f64, t5278: f64, t5283: f64, t546: f64, t56275: f64, t56486: f64, t6347: f64, t6404: f64, t6408: f64, t6411: f64) -> f64 {
    let t57298 = 60.0_f64 * t1995 * t3734 * t5278 * t6347 + 3.0_f64 * t1347 * t546 * t56275 + 240.0_f64 * t16148 * t16191 * t5278 - 24.0_f64 * t3843 * t546 * t56486 + 6.0_f64 * t1345 * t19728 + 6.0_f64 * t1348 * t19702 + 6.0_f64 * t16176 * t1821 - 24.0_f64 * t16186 * t19725 + 6.0_f64 * t16202 * t1819 - 12.0_f64 * t3839 * t6408 + 3.0_f64 * t3839 * t6411 + 3.0_f64 * t3847 * t6404 + 12.0_f64 * t5272 * t5283;
    t57298
}
