//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1122/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1122<F: Float>(t2034: F, t2124: F, t2126: F, t2168: F, t29210: F, t29441: F, t3501: F, t38433: F, t38444: F, t38446: F, t49070: F, t49072: F, t49106: F, t49142: F, t49144: F, t49172: F, t56391: F, t56467: F, t56501: F, t6931: F) -> (F,) {
    let t56631 = 0.16227234780939014661e1 * t49070 + 0.2821319449668395048e0 * t49072 - 0.21764464326013333228e1 * t3501 * t2034 * t56467 - 0.36274107210022222046e1 * t2168 * t6931 * t56501 + 0.21316635841938984807e2 * t29210 - 0.48681704342817043985e1 * t49106 - 0.25391875047015555432e1 * t49142 + 0.48681704342817043984e1 * t49144 + 0.10658317920969492404e2 * t29441 - 0.47962430644362715816e1 * t38433 + 0.20863587575493018851e1 * t2124 * t2126 * t56391 + 0.19472681737126817594e2 * t49172 - 0.23981215322181357908e1 * t38444 - 0.23981215322181357908e1 * t38446;
    (t56631,)
}
