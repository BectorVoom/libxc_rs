//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 989/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk989<F: Float>(t1024: F, t12176: F, t14431: F, t1841: F, t1843: F, t1897: F, t1901: F, t2508: F, t3732: F, t45029: F, t45031: F, t45034: F, t45037: F, t45044: F, t45048: F, t45052: F, t45054: F, t45057: F, t45059: F, t45062: F, t47702: F, t47731: F, t50063: F, t50118: F, t7129: F, t8942: F) -> F {
    let t50454 = F::new(0.85450291446024714263e-3) * t1841 * t1843 * t50118 - F::new(0.17090058289204942853e-2) * t47702 + F::new(0.15381052460284448567e-1) * t7129 * t14431 + F::new(0.15381052460284448567e-1) * t2508 * t12176 * t1024 - F::new(0.15381052460284448567e-1) * t1897 * t3732 * t8942 - t45029 + t45031 - t45034 + t45037 + t45044 + t45048 + F::new(0.76905262301422242837e-2) * t1897 * t1901 * t50063 + F::new(0.1281754371690370714e-2) * t47731 + t45052 - F::new(0.64087718584518535698e-3) * t45054 - t45057 + t45059 + t45062;
    t50454
}
