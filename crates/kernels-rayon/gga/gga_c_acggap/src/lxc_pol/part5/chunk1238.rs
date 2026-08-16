//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1238/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1238(t1180: f64, t1181: f64, t13582: f64, t174: f64, t22607: f64, t22613: f64, t22617: f64, t22619: f64, t22621: f64, t22623: f64, t22625: f64, t22627: f64, t22633: f64, t22642: f64, t336: f64, t367: f64, t372: f64, t386: f64, t418: f64, t428: f64, t5207: f64, t5867: f64, t6119: f64) -> f64 {
    let t22644 = 0.42874018118069736972e-3_f64 * t418 * t386 * t428 * t174 * t22607 - 0.85748036236139473944e-3_f64 * t22613 - 0.42874018118069736972e-3_f64 * t22617 - 0.11337795902333997111e-1_f64 * t22619 - 0.80031500487063509015e-2_f64 * t22621 + 0.11337795902333997111e0_f64 * t22623 + 0.16006300097412701803e-1_f64 * t22625 - 0.34299214494455789578e-2_f64 * t22627 - t367 * t336 * t6119 * t372 / 48.0_f64 - 0.80031500487063509014e-2_f64 * t22633 + 0.17149607247227894789e-2_f64 * t1180 * t1181 * t5867 * t5207 + 0.21437009059034868486e-3_f64 * t13582 + 0.85748036236139473944e-3_f64 * t22642;
    t22644
}
