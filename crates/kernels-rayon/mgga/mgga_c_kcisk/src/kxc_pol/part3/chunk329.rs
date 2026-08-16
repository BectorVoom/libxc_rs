//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 329/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk329(t227: f64, t1456: f64, t1521: f64, t1607: f64, t1611: f64, t1620: f64, t240: f64, t555: f64, t297: f64, t1060: f64, t565: f64, t298: f64, t430: f64, t569: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t228 = t227 <= zeta_threshold;
    let t1624 = t1456 - t1521 + t240 * (t1607 * t555 - t1611 * t1620 - t1456 + t1521);
    let t1625 = t297 * t1624;
    let t1628 = piecewise3(t228, 0.0_f64, t1060);
    let t1629 = t565 * t1628;
    let t1634 = t298 * t430 * t569;
    (t1624, t1625, t1629, t1634)
}
