//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2763/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2763(t1509: f64, t4265: f64, t13336: f64, t13393: f64, t13450: f64, t13453: f64, t1510: f64, t1525: f64, t16756: f64, t16758: f64, t16815: f64, t16817: f64, t16820: f64, t16825: f64, t16830: f64, t17031: f64, t17034: f64, t2617: f64, t2679: f64, t2684: f64, t4291: f64, t47395: f64, t47419: f64, t5651: f64, t812: f64, t829: f64, t9612: f64) -> (f64, f64) {
    let t58204 = t4265 * t1509;
    let t58224 = -2.0_f64 * t1510 * t47395 * t812 - 2.0_f64 * t16758 * t2684 * t4291 - t16815 * t2679 * t4291 - 4.0_f64 * t4291 * t58204 * t829 + 2.0_f64 * t13336 * t1525 + 8.0_f64 * t13393 * t17034 - 2.0_f64 * t13450 * t16830 + 8.0_f64 * t13453 * t16820 + 12.0_f64 * t13453 * t16825 + 4.0_f64 * t13453 * t17031 - 2.0_f64 * t16756 * t2617 - 12.0_f64 * t16817 * t47419 - t5651 * t9612;
    (t58204, t58224)
}
