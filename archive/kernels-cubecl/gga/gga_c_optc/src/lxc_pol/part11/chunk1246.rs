//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1246/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1246<F: Float>(t2034: F, t2124: F, t2126: F, t2168: F, t29210: F, t29441: F, t3501: F, t38433: F, t38444: F, t38446: F, t49070: F, t49072: F, t49106: F, t49142: F, t49144: F, t49172: F, t56391: F, t56467: F, t56501: F, t6931: F) -> F {
    let t56631 = F::cast_from(0.16227234780939014661e1_f64) * t49070 + F::cast_from(0.2821319449668395048e0_f64) * t49072 - F::cast_from(0.21764464326013333228e1_f64) * t3501 * t2034 * t56467 - F::cast_from(0.36274107210022222046e1_f64) * t2168 * t6931 * t56501 + F::cast_from(0.21316635841938984807e2_f64) * t29210 - F::cast_from(0.48681704342817043985e1_f64) * t49106 - F::cast_from(0.25391875047015555432e1_f64) * t49142 + F::cast_from(0.48681704342817043984e1_f64) * t49144 + F::cast_from(0.10658317920969492404e2_f64) * t29441 - F::cast_from(0.47962430644362715816e1_f64) * t38433 + F::cast_from(0.20863587575493018851e1_f64) * t2124 * t2126 * t56391 + F::cast_from(0.19472681737126817594e2_f64) * t49172 - F::cast_from(0.23981215322181357908e1_f64) * t38444 - F::cast_from(0.23981215322181357908e1_f64) * t38446;
    t56631
}
