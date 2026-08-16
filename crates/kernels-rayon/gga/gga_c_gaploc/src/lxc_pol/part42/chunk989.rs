//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 989/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk989(t1024: f64, t12176: f64, t14431: f64, t1841: f64, t1843: f64, t1897: f64, t1901: f64, t2508: f64, t3732: f64, t45029: f64, t45031: f64, t45034: f64, t45037: f64, t45044: f64, t45048: f64, t45052: f64, t45054: f64, t45057: f64, t45059: f64, t45062: f64, t47702: f64, t47731: f64, t50063: f64, t50118: f64, t7129: f64, t8942: f64) -> f64 {
    let t50454 = 0.85450291446024714263e-3_f64 * t1841 * t1843 * t50118 - 0.17090058289204942853e-2_f64 * t47702 + 0.15381052460284448567e-1_f64 * t7129 * t14431 + 0.15381052460284448567e-1_f64 * t2508 * t12176 * t1024 - 0.15381052460284448567e-1_f64 * t1897 * t3732 * t8942 - t45029 + t45031 - t45034 + t45037 + t45044 + t45048 + 0.76905262301422242837e-2_f64 * t1897 * t1901 * t50063 + 0.1281754371690370714e-2_f64 * t47731 + t45052 - 0.64087718584518535698e-3_f64 * t45054 - t45057 + t45059 + t45062;
    t50454
}
