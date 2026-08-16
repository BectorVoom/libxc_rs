//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2693/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2693(t12171: f64, t1336: f64, t1352: f64, t16060: f64, t16132: f64, t1840: f64, t19658: f64, t19660: f64, t19752: f64, t19805: f64, t19815: f64, t20495: f64, t20648: f64, t26322: f64, t3777: f64, t5234: f64, t5339: f64, t5341: f64, t5344: f64, t6420: f64, t6454: f64, t74967: f64) -> f64 {
    let t75101 = 6.0_f64 * t12171 * t1336 * t20495 - 3.0_f64 * t1336 * t16132 * t6420 - t1352 * t5344 * t74967 - 3.0_f64 * t19660 * t26322 * t5344 - 3.0_f64 * t16060 * t6454 + 3.0_f64 * t1840 * t19805 - 3.0_f64 * t19658 * t5234 - 6.0_f64 * t19752 * t5234 - 3.0_f64 * t19815 * t5339 - 3.0_f64 * t19815 * t5341 - 3.0_f64 * t20648 * t3777;
    t75101
}
