//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2108/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2108(t2132: f64, t24746: f64, t95413: f64, t3545: f64, t8020: f64, t1202: f64, t27603: f64, t24736: f64, t4993: f64, t15486: f64, t7345: f64, t27599: f64, t3572: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t95446 = 0.20186378047070195428e-3_f64 * t2132 * t95413 * t24746;
    let t95450 = t8020 * t3545;
    let t95452 = t1202 * t27603;
    let t95456 = t24736 * t4993 / 1728.0_f64;
    let t95459 = t7345 * t15486 / 1728.0_f64;
    let t95463 = t27599 * t3572 / 216.0_f64;
    (t95446, t95450, t95452, t95456, t95459, t95463)
}
