//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1238/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1238<F: Float>(t1180: F, t1181: F, t13582: F, t174: F, t22607: F, t22613: F, t22617: F, t22619: F, t22621: F, t22623: F, t22625: F, t22627: F, t22633: F, t22642: F, t336: F, t367: F, t372: F, t386: F, t418: F, t428: F, t5207: F, t5867: F, t6119: F) -> F {
    let t22644 = F::cast_from(0.42874018118069736972e-3_f64) * t418 * t386 * t428 * t174 * t22607 - F::cast_from(0.85748036236139473944e-3_f64) * t22613 - F::cast_from(0.42874018118069736972e-3_f64) * t22617 - F::cast_from(0.11337795902333997111e-1_f64) * t22619 - F::cast_from(0.80031500487063509015e-2_f64) * t22621 + F::cast_from(0.11337795902333997111e0_f64) * t22623 + F::cast_from(0.16006300097412701803e-1_f64) * t22625 - F::cast_from(0.34299214494455789578e-2_f64) * t22627 - t367 * t336 * t6119 * t372 / F::cast_from(48.0_f64) - F::cast_from(0.80031500487063509014e-2_f64) * t22633 + F::cast_from(0.17149607247227894789e-2_f64) * t1180 * t1181 * t5867 * t5207 + F::cast_from(0.21437009059034868486e-3_f64) * t13582 + F::cast_from(0.85748036236139473944e-3_f64) * t22642;
    t22644
}
