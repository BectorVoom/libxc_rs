//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1161/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1161(t1353: f64, t23767: f64, t31543: f64, t1061: f64, t424: f64, t481: f64, t6603: f64, t7974: f64, t10167: f64, t1358: f64, t30204: f64, t6525: f64, t7967: f64) -> (f64, f64, f64, f64, f64) {
    let t31546 = 0.63233348079280332442e-2_f64 * t23767 * t31543 * t1353;
    let t31548 = t481 * t1061 * t424;
    let t31551 = 0.56910013271352299198e-1_f64 * t31548 * t6603 * t7974;
    let t31552 = t1358 * t10167;
    let t31553 = 0.94850022118920498665e-2_f64 * t31552;
    let t31555 = t6525 * t30204 * t7967;
    (t31546, t31548, t31551, t31553, t31555)
}
