//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1108/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1108<F: Float>(t41041: F, t41057: F, t40993: F, t41001: F, t41004: F, t41021: F, t41024: F, t41029: F, t41033: F, t41037: F, t41045: F, t41049: F, t41053: F) -> F {
    let t44110 = F::new(0.36366215538993788974e-1) * t41041;
    let t44114 = F::new(0.10909864661698136692e0) * t41057;
    let t44115 = F::new(0.35922725105591425692e0) * t40993 - F::new(0.8182398496273602519e0) * t41001 - F::new(0.16364796992547205038e0) * t41004 + F::new(0.66671395154821946452e-1) * t41021 - F::new(0.40911992481368012596e-1) * t41024 - F::new(0.20001418546446583936e0) * t41029 + F::new(0.26668558061928778581e0) * t41033 - F::new(0.14546486215597515589e0) * t41037 - t44110 - F::new(0.20455996240684006298e-1) * t41045 + F::new(0.2727466165424534173e-1) * t41049 + F::new(0.68186654135613354325e-2) * t41053 + t44114;
    t44115
}
