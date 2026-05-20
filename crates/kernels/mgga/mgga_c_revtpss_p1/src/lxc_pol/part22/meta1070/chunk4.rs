//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3831/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3831<F: Float>(t47466: F, t47474: F, t47478: F, t47483: F, t47487: F, t47495: F, t47497: F, t47500: F, t47837: F, t47839: F, t47844: F, t47857: F, t47860: F, t47863: F, t73641: F, t73647: F, t73652: F, t73656: F, t73662: F) -> F {
    let t73664 = -F::cast_from(0.11565819519348392139e-2_f64) * t47466 - F::cast_from(0.46263278077393568556e-2_f64) * t47837 + F::cast_from(0.58537326070537880875e-1_f64) * t47839 + F::cast_from(0.92526556154787137113e-2_f64) * t47844 + F::cast_from(0.65049603595885220126e-3_f64) * t73641 + F::cast_from(0.60712963356159538784e-1_f64) * t47474 - F::cast_from(0.60712963356159538784e-1_f64) * t47478 + F::cast_from(0.23131639038696784278e-2_f64) * t47483 + F::cast_from(0.92526556154787137112e-2_f64) * t47487 - F::cast_from(0.19514881078765566038e-1_f64) * t73647 - F::cast_from(0.46263278077393568556e-2_f64) * t47857 - F::cast_from(0.1040793657534163522e0_f64) * t47860 + F::cast_from(0.21951497276451705328e-1_f64) * t73652 + F::cast_from(0.60712963356159538786e-1_f64) * t47863 + F::cast_from(0.11565819519348392139e-2_f64) * t73656 - F::cast_from(0.52039682876708176102e-2_f64) * t47495 + F::cast_from(0.34146773541147097178e-1_f64) * t47497 + F::cast_from(0.13009920719177044025e-2_f64) * t47500 + F::cast_from(0.13009920719177044025e-1_f64) * t73662;
    t73664
}
