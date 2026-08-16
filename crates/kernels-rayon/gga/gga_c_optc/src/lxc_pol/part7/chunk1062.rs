//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1062/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1062(t2160: f64, t23071: f64, t6997: f64, t7018: f64, t155: f64, t652: f64, t6991: f64, t6994: f64, t127: f64, t2024: f64, t2034: f64, t2113: f64, t2126: f64, t2168: f64, t22769: f64, t22787: f64, t22970: f64, t22979: f64, t23028: f64, t23040: f64, t23045: f64, t23050: f64, t23052: f64, t23066: f64, t23068: f64, t3467: f64, t5: f64, t673: f64, t675: f64, t6879: f64, t7002: f64) -> f64 {
    let t23072 = t23071 * t2160;
    let t23074 = t7018 * t6997;
    let t23077 = t155 * t6991 * t652;
    let t23078 = t23077 * t6994;
    let t23080 = 0.81136173904695073307e1_f64 * t23028 + 0.12170426085704260996e1_f64 * t2113 * t675 * t22979 * t2024 - 0.31295381363239528276e1_f64 * t7002 * t675 * t22979 * t6879 + 0.20863587575493018851e1_f64 * t23040 * t675 * t22979 * t22787 + 0.2821319449668395048e0_f64 * t23045 + t23050 + 0.72548214420044444092e0_f64 * t2168 * t2034 * t23052 - 0.417271751509860377e1_f64 * t3467 * t2126 * t22970 - 0.86931614897887578546e-1_f64 * t673 * t675 * t5 * t22769 * t127 + 0.11719669564011510589e2_f64 * t23066 - 0.40568086952347536654e1_f64 * t23068 + 0.71943645966544073724e1_f64 * t23072 - 0.25391875047015555432e1_f64 * t23074 + 0.33855833396020740576e1_f64 * t23078;
    t23080
}
