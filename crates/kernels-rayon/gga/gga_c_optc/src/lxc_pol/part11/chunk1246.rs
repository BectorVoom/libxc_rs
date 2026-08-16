//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1246/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1246(t2034: f64, t2124: f64, t2126: f64, t2168: f64, t29210: f64, t29441: f64, t3501: f64, t38433: f64, t38444: f64, t38446: f64, t49070: f64, t49072: f64, t49106: f64, t49142: f64, t49144: f64, t49172: f64, t56391: f64, t56467: f64, t56501: f64, t6931: f64) -> f64 {
    let t56631 = 0.16227234780939014661e1_f64 * t49070 + 0.2821319449668395048e0_f64 * t49072 - 0.21764464326013333228e1_f64 * t3501 * t2034 * t56467 - 0.36274107210022222046e1_f64 * t2168 * t6931 * t56501 + 0.21316635841938984807e2_f64 * t29210 - 0.48681704342817043985e1_f64 * t49106 - 0.25391875047015555432e1_f64 * t49142 + 0.48681704342817043984e1_f64 * t49144 + 0.10658317920969492404e2_f64 * t29441 - 0.47962430644362715816e1_f64 * t38433 + 0.20863587575493018851e1_f64 * t2124 * t2126 * t56391 + 0.19472681737126817594e2_f64 * t49172 - 0.23981215322181357908e1_f64 * t38444 - 0.23981215322181357908e1_f64 * t38446;
    t56631
}
