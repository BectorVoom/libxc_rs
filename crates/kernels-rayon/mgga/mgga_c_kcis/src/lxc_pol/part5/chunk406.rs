//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 406/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk406(t1489: f64, t556: f64, t572: f64, t1533: f64, t1494: f64, t1497: f64, t571: f64, t1457: f64, t552: f64, t577: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1534 = t556 * t1489;
    let t1535 = t572 * t1534;
    let t1536 = t1533 * t1535;
    let t1538 = t1494 * t1497;
    let t1539 = t572 * t1538;
    let t1540 = t571 * t1539;
    let t1542 = t1457 * t552;
    let t1543 = t1542 * t577;
    (t1534, t1535, t1536, t1538, t1539, t1540, t1542, t1543)
}
