//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 976/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk976<F: Float>(t146: F, t6567: F, t671: F, t678: F, t2132: F, t7030: F, t155: F, t2078: F, t2157: F, t2160: F, t6997: F, t7018: F, t652: F, t6991: F, t6994: F, t127: F, t2024: F, t2034: F, t2113: F, t2126: F, t2168: F, t22769: F, t22787: F, t22970: F, t22979: F, t23028: F, t23040: F, t23045: F, t23050: F, t23052: F, t3467: F, t5: F, t673: F, t675: F, t6879: F, t7002: F) -> (F,) {
    let t23065 = t146 * t671 * t6567;
    let t23066 = t23065 * t678;
    let t23068 = t7030 * t2132;
    let t23071 = t155 * t2157 * t2078;
    let t23072 = t23071 * t2160;
    let t23074 = t7018 * t6997;
    let t23077 = t155 * t6991 * t652;
    let t23078 = t23077 * t6994;
    let t23080 = 0.81136173904695073307e1 * t23028 + 0.12170426085704260996e1 * t2113 * t675 * t22979 * t2024 - 0.31295381363239528276e1 * t7002 * t675 * t22979 * t6879 + 0.20863587575493018851e1 * t23040 * t675 * t22979 * t22787 + 0.2821319449668395048e0 * t23045 + t23050 + 0.72548214420044444092e0 * t2168 * t2034 * t23052 - 0.417271751509860377e1 * t3467 * t2126 * t22970 - 0.86931614897887578546e-1 * t673 * t675 * t5 * t22769 * t127 + 0.11719669564011510589e2 * t23066 - 0.40568086952347536654e1 * t23068 + 0.71943645966544073724e1 * t23072 - 0.25391875047015555432e1 * t23074 + 0.33855833396020740576e1 * t23078;
    (t23080,)
}
