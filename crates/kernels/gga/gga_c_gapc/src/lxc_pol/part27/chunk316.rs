//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 316/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk316<F: Float>(t402: F, t78: F, t4: F, t3: F, t95: F, t545: F, t106: F, t391: F, t1147: F, t1174: F, t550: F, t10: F, t103: F, t1263: F, t1343: F, t1347: F, t1353: F, t1356: F, t1360: F, t160: F, t161: F, t164: F, t421: F, t540: F, t544: F, t547: F, t551: F, t79: F, t99: F) -> (F, F) {
    let t1361 = t78 * t402;
    let t1362 = t4 * t1361;
    let t1365 = t3 * t95;
    let t1366 = t545 * t1365;
    let t1369 = t106 * t391;
    let t1370 = t1369 * t1147;
    let t1373 = t550 * t1174;
    let t1385 = 0.619125e-2 * t1343 * t161 - 0.24765e-1 * t1347 * t547 - 0.123825e-1 * t540 * t551 + 0.206375e-2 * t1353 * t1356 + 0.24765e-1 * t1360 * t1362 + 0.1651e-1 * t544 * t1366 + 0.123825e-1 * t160 * t1370 - 0.619125e-2 * t160 * t1373 + 0.17687407407407407407e-1 * t103 * t79 * t99 - 0.10612444444444444444e0 * t103 * t10 * t421 - 0.79593333333333333331e-1 * t103 * t164 * t1263;
    (t1365, t1385)
}
