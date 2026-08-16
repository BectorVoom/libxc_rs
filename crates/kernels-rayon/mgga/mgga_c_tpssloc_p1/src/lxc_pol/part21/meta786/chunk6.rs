//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2732/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2732(t1352: f64, t5286: f64, t3787: f64, t6434: f64, t1338: f64, t20009: f64, t1336: f64, t1381: f64, t16133: f64, t16206: f64, t16414: f64, t1814: f64, t1838: f64, t19657: f64, t19815: f64, t3793: f64, t3851: f64, t3898: f64, t3902: f64, t5230: f64, t5234: f64, t5335: f64, t5344: f64, t5348: f64, t5351: f64, t53909: f64, t544: f64, t553: f64, t56923: f64, t57485: f64) -> f64 {
    let t57643 = t1352 * t5286;
    let t57653 = t3787 * t6434;
    let t57659 = t1338 * t20009;
    let t57667 = -2.0_f64 * t1336 * t1352 * t57659 - 2.0_f64 * t1336 * t16206 * t5348 - t1336 * t19657 * t3851 + 2.0_f64 * t1336 * t3793 * t57653 - 4.0_f64 * t5335 * t5344 * t57643 + t544 * t553 * t57485 - 2.0_f64 * t1381 * t56923 - 4.0_f64 * t16133 * t5234 + 2.0_f64 * t16414 * t1814 - 2.0_f64 * t1838 * t53909 + 2.0_f64 * t19815 * t3898 - 2.0_f64 * t19815 * t3902 + 4.0_f64 * t5230 * t5351;
    t57667
}
