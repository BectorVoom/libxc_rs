//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1366/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1366(t12002: f64, t12012: f64, t12019: f64, t1212: f64, t1349: f64, t1353: f64, t1358: f64, t1359: f64, t13777: f64, t161: f64, t31646: f64, t31652: f64, t31660: f64, t31662: f64, t31672: f64, t31674: f64, t31679: f64, t31681: f64, t31685: f64, t3692: f64, t380: f64, t419: f64, t488: f64) -> f64 {
    let t38354 = 0.56910013271352299198e-1_f64 * t419 * t12019 - 0.7588001769513639893e-1_f64 * t380 * t12002 + 0.28455006635676149599e-1_f64 * t1212 * t3692 - 0.63233348079280332442e-2_f64 * t1358 * t1359 * t13777 * t488 + 0.63233348079280332442e-2_f64 * t1349 * t12012 * t161 * t1353 - t31646 - t31652 - t31660 + t31662 - t31672 - t31674 + t31679 + t31681 + t31685;
    t38354
}
