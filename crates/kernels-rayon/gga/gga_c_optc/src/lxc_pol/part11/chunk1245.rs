//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1245/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1245(t116: f64, t2010: f64, t22895: f64, t23136: f64, t29128: f64, t38074: f64, t38105: f64, t38107: f64, t38148: f64, t38172: f64, t38174: f64, t49035: f64, t49046: f64, t55893: f64, t56222: f64, t56224: f64, t56229: f64, t686: f64, t705: f64) -> f64 {
    let t56610 = 0.5642638899336790096e0_f64 * t49035 + 0.81136173904695073307e1_f64 * t38074 + 0.23439339128023021177e2_f64 * t29128 - 0.40568086952347536654e1_f64 * t38105 + 0.71943645966544073724e1_f64 * t38107 + 0.10431793787746509426e2_f64 * t686 * t22895 * t116 * t56222 + 0.15647690681619764138e1_f64 * t686 * t2010 * t116 * t55893 + 0.45342634012527777558e0_f64 * t705 * t56229 + 0.63479687617538888581e1_f64 * t705 * t56224 + 0.16927916698010370288e2_f64 * t49046 - 0.40568086952347536654e1_f64 * t38148 + t23136 - 0.81136173904695073307e1_f64 * t38172 + 0.24340852171408521992e2_f64 * t38174;
    t56610
}
