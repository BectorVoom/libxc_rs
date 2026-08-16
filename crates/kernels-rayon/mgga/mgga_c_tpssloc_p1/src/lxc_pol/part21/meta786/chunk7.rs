//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2733/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2733(t12259: f64, t12267: f64, t1332: f64, t1336: f64, t16127: f64, t16132: f64, t16423: f64, t19657: f64, t19658: f64, t19813: f64, t19815: f64, t20010: f64, t3773: f64, t3777: f64, t3856: f64, t3905: f64, t3907: f64, t40486: f64, t5234: f64, t5287: f64, t6388: f64, t6415: f64, t6456: f64, t6458: f64) -> f64 {
    let t57692 = -t12259 * t1336 * t6415 - 4.0_f64 * t1336 * t16132 * t5287 - t1336 * t19657 * t3856 + 2.0_f64 * t1336 * t40486 * t6388 - t12267 * t6456 + 2.0_f64 * t1332 * t20010 - 2.0_f64 * t16127 * t5234 - 2.0_f64 * t16423 * t5234 - 2.0_f64 * t19658 * t3777 - 2.0_f64 * t19813 * t3777 - t19815 * t3905 - t19815 * t3907 + t3773 * t6458;
    t57692
}
