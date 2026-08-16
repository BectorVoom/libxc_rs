//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 703/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk703(t1043: f64, t2775: f64, t2770: f64, t3061: f64, t1022: f64, t3131: f64, t3188: f64, t1932: f64, t360: f64, t193: f64, t336: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4583 = t1043 * t2775;
    let t4588 = t3061 * t2770;
    let t4594 = t3131 * t1022;
    let t4673 = t3188 * t1022;
    let t4684 = t1932 * t1022 * t360;
    let t4700 = t193 * t336;
    (t4583, t4588, t4594, t4673, t4684, t4700)
}
