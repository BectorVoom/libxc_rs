//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1379/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1379(t12148: f64, t1377: f64, t13777: f64, t493: f64, t1339: f64, t1436: f64, t1441: f64, t1537: f64, t1540: f64, t18383: f64, t34025: f64, t34032: f64, t34036: f64, t34038: f64, t34052: f64, t34054: f64, t34056: f64, t34058: f64, t34061: f64, t34065: f64, t34067: f64, t4130: f64, t4781: f64) -> (f64, f64) {
    let t38458 = 2.0_f64 * t1377 * t12148;
    let t38463 = t493 * t13777;
    let t38474 = -t34025 + 0.30674340763136599742e1_f64 * t4781 * t4130 * t13777 * t1540 - 0.1022478025437886658e1_f64 * t1436 * t38463 * t18383 + 0.2044956050875773316e1_f64 * t1441 * t38463 * t1540 - 0.51123901271894332902e1_f64 * t1537 * t1339 * t13777 * t1540 - t34032 - t34036 - t34038 - t34052 - t34054 - t34056 + t34058 + t34061 - t34065 + t34067;
    (t38458, t38474)
}
