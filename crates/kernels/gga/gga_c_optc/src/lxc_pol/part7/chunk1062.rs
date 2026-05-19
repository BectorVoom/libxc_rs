//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1062/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1062<F: Float>(t2160: F, t23071: F, t6997: F, t7018: F, t155: F, t652: F, t6991: F, t6994: F, t127: F, t2024: F, t2034: F, t2113: F, t2126: F, t2168: F, t22769: F, t22787: F, t22970: F, t22979: F, t23028: F, t23040: F, t23045: F, t23050: F, t23052: F, t23066: F, t23068: F, t3467: F, t5: F, t673: F, t675: F, t6879: F, t7002: F) -> F {
    let t23072 = t23071 * t2160;
    let t23074 = t7018 * t6997;
    let t23077 = t155 * t6991 * t652;
    let t23078 = t23077 * t6994;
    let t23080 = F::cast_from(0.81136173904695073307e1_f64) * t23028 + F::cast_from(0.12170426085704260996e1_f64) * t2113 * t675 * t22979 * t2024 - F::cast_from(0.31295381363239528276e1_f64) * t7002 * t675 * t22979 * t6879 + F::cast_from(0.20863587575493018851e1_f64) * t23040 * t675 * t22979 * t22787 + F::cast_from(0.2821319449668395048e0_f64) * t23045 + t23050 + F::cast_from(0.72548214420044444092e0_f64) * t2168 * t2034 * t23052 - F::cast_from(0.417271751509860377e1_f64) * t3467 * t2126 * t22970 - F::cast_from(0.86931614897887578546e-1_f64) * t673 * t675 * t5 * t22769 * t127 + F::cast_from(0.11719669564011510589e2_f64) * t23066 - F::cast_from(0.40568086952347536654e1_f64) * t23068 + F::cast_from(0.71943645966544073724e1_f64) * t23072 - F::cast_from(0.25391875047015555432e1_f64) * t23074 + F::cast_from(0.33855833396020740576e1_f64) * t23078;
    t23080
}
