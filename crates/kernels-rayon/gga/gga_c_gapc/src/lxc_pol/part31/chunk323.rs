//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 323/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk323(t106: f64, t391: f64, t1147: f64, t1174: f64, t550: f64, t10: f64, t103: f64, t1263: f64, t1343: f64, t1347: f64, t1353: f64, t1356: f64, t1360: f64, t1362: f64, t1366: f64, t160: f64, t161: f64, t164: f64, t421: f64, t540: f64, t544: f64, t547: f64, t551: f64, t79: f64, t99: f64) -> f64 {
    let t1369 = t106 * t391;
    let t1370 = t1369 * t1147;
    let t1373 = t550 * t1174;
    let t1385 = 0.619125e-2_f64 * t1343 * t161 - 0.24765e-1_f64 * t1347 * t547 - 0.123825e-1_f64 * t540 * t551 + 0.206375e-2_f64 * t1353 * t1356 + 0.24765e-1_f64 * t1360 * t1362 + 0.1651e-1_f64 * t544 * t1366 + 0.123825e-1_f64 * t160 * t1370 - 0.619125e-2_f64 * t160 * t1373 + 0.17687407407407407407e-1_f64 * t103 * t79 * t99 - 0.10612444444444444444e0_f64 * t103 * t10 * t421 - 0.79593333333333333331e-1_f64 * t103 * t164 * t1263;
    t1385
}
