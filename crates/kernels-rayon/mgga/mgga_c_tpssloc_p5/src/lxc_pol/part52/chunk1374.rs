//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1374/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1374(t116152: f64, t120121: f64, t123023: f64, t123050: f64, t123052: f64, t123054: f64, t123056: f64, t123058: f64, t123060: f64, t123062: f64, t123067: f64, t123072: f64, t1458: f64, t31237: f64, t31239: f64, t31880: f64, t33152: f64, t33154: f64, t4072: f64, t671: f64, t8446: f64) -> f64 {
    let t123074 = 2.0_f64 * t116152 * t1458 + 2.0_f64 * t123062 * t671 + 2.0_f64 * t123067 * t1458 + 2.0_f64 * t31880 * t4072 + t120121 + t123023 + 2.0_f64 * t123050 + 2.0_f64 * t123052 + 2.0_f64 * t123054 + 2.0_f64 * t123056 + 2.0_f64 * t123058 + 2.0_f64 * t123060 + 2.0_f64 * t123072 + t31237 + t31239 + t33152 + t33154 + t8446;
    t123074
}
