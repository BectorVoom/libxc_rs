//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1245/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1245<F: Float>(t116: F, t2010: F, t22895: F, t23136: F, t29128: F, t38074: F, t38105: F, t38107: F, t38148: F, t38172: F, t38174: F, t49035: F, t49046: F, t55893: F, t56222: F, t56224: F, t56229: F, t686: F, t705: F) -> F {
    let t56610 = F::cast_from(0.5642638899336790096e0_f64) * t49035 + F::cast_from(0.81136173904695073307e1_f64) * t38074 + F::cast_from(0.23439339128023021177e2_f64) * t29128 - F::cast_from(0.40568086952347536654e1_f64) * t38105 + F::cast_from(0.71943645966544073724e1_f64) * t38107 + F::cast_from(0.10431793787746509426e2_f64) * t686 * t22895 * t116 * t56222 + F::cast_from(0.15647690681619764138e1_f64) * t686 * t2010 * t116 * t55893 + F::cast_from(0.45342634012527777558e0_f64) * t705 * t56229 + F::cast_from(0.63479687617538888581e1_f64) * t705 * t56224 + F::cast_from(0.16927916698010370288e2_f64) * t49046 - F::cast_from(0.40568086952347536654e1_f64) * t38148 + t23136 - F::cast_from(0.81136173904695073307e1_f64) * t38172 + F::cast_from(0.24340852171408521992e2_f64) * t38174;
    t56610
}
