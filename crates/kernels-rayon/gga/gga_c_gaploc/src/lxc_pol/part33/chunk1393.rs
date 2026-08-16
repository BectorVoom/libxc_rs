//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1393/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1393(t1359: f64, t3689: f64, t12051: f64, t1415: f64, t1429: f64, t1457: f64, t1572: f64, t1617: f64, t1646: f64, t34720: f64, t34726: f64, t34730: f64, t34733: f64, t34737: f64, t34740: f64, t34743: f64, t34746: f64, t34749: f64, t34752: f64, t34762: f64, t34766: f64, t38399: f64, t38447: f64, t549: f64) -> f64 {
    let t38674 = t1359 * t3689;
    let t38678 = t34720 - t34726 + t34730 + t34733 - t34737 + 0.46011511144704899612e1_f64 * t1617 * t12051 + 0.71500979903700853338e0_f64 * t1572 * t1457 * t38399 + 0.79445533226334281486e-1_f64 * t1429 * t549 * t38447 - t34740 + t34743 + t34746 - t34749 - t34752 - t34762 + t34766 - 0.71500979903700853338e0_f64 * t1415 * t38674 * t1646;
    t38678
}
