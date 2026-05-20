//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3909/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3909<F: Float>(t10014: F, t22332: F, t22351: F, t2439: F, t2777: F, t22253: F, t4101: F, t686: F, t72: F, t22335: F, t2470: F, t14122: F, t22321: F, t4057: F, t46520: F, t46526: F, t49167: F, t49172: F, t49176: F, t49178: F, t49186: F, t49189: F, t5659: F, t5755: F, t820: F) -> F {
    let t75071 = t10014 * t22332;
    let t75074 = t2439 * t2777 * t22351;
    let t75089 = t4101 * t22253 * t72 * t686;
    let t75092 = t4101 * t22335 * t2470;
    let t75097 = -F::cast_from(0.19514881078765566038e-1_f64) * t75071 - F::cast_from(0.65049603595885220126e-3_f64) * t75074 - F::cast_from(0.26341796731742046394e1_f64) * t5755 * t14122 * t5659 - F::cast_from(0.13009920719177044025e-1_f64) * t46520 + F::cast_from(0.22089088168956307394e-3_f64) * t46526 + F::cast_from(0.2601984143835408805e-2_f64) * t49167 - F::cast_from(0.34146773541147097178e-1_f64) * t49172 - F::cast_from(0.29268663035268940438e-1_f64) * t49176 + F::cast_from(0.34146773541147097178e-1_f64) * t49178 + F::cast_from(0.92526556154787137113e-2_f64) * t49186 - F::cast_from(0.46263278077393568556e-2_f64) * t49189 - F::cast_from(0.19514881078765566038e-1_f64) * t75089 + F::cast_from(0.13009920719177044025e-1_f64) * t75092 - F::cast_from(0.65854491829355115987e0_f64) * t820 * t22321 * t4057;
    t75097
}
