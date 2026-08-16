//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1426/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1426(t34407: f64, t6710: f64, t6711: f64, t1429: f64, t2365: f64, t2366: f64, t7861: f64, t18970: f64, t3381: f64, t1: f64, t1559: f64, t544: f64, t986: f64) -> (f64, f64, f64, f64) {
    let t35192 = 0.23005755572352449806e2_f64 * t6710 * t6711 * t34407;
    let t35198 = t1429 * t2365 * t2366 * t7861;
    let t35199 = 0.14896037479937677779e-1_f64 * t35198;
    let t35200 = t18970 * t3381;
    let t35201 = 0.14896037479937677779e-1_f64 * t35200;
    let t35204 = t544 * t1559 * t986 * t1;
    (t35192, t35199, t35201, t35204)
}
