//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1296/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1296(t3391: f64, t4634: f64, t10371: f64, t1641: f64, t10385: f64, t1580: f64, t1445: f64, t31857: f64, t597: f64, t2778: f64, t574: f64, t6393: f64) -> (f64, f64, f64, f64, f64) {
    let t34054 = 0.46011511144704899612e1_f64 * t4634 * t3391;
    let t34056 = 0.92023022289409799224e1_f64 * t1641 * t10371;
    let t34058 = 0.23005755572352449806e2_f64 * t1580 * t10385;
    let t34061 = 0.11502877786176224903e2_f64 * t597 * t1445 * t31857;
    let t34065 = 0.46011511144704899612e1_f64 * t574 * t1445 * t2778 * t6393;
    (t34054, t34056, t34058, t34061, t34065)
}
