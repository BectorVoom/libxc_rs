//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3909/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3909(t10014: f64, t22332: f64, t22351: f64, t2439: f64, t2777: f64, t22253: f64, t4101: f64, t686: f64, t72: f64, t22335: f64, t2470: f64, t14122: f64, t22321: f64, t4057: f64, t46520: f64, t46526: f64, t49167: f64, t49172: f64, t49176: f64, t49178: f64, t49186: f64, t49189: f64, t5659: f64, t5755: f64, t820: f64) -> f64 {
    let t75071 = t10014 * t22332;
    let t75074 = t2439 * t2777 * t22351;
    let t75089 = t4101 * t22253 * t72 * t686;
    let t75092 = t4101 * t22335 * t2470;
    let t75097 = -0.19514881078765566038e-1_f64 * t75071 - 0.65049603595885220126e-3_f64 * t75074 - 0.26341796731742046394e1_f64 * t5755 * t14122 * t5659 - 0.13009920719177044025e-1_f64 * t46520 + 0.22089088168956307394e-3_f64 * t46526 + 0.2601984143835408805e-2_f64 * t49167 - 0.34146773541147097178e-1_f64 * t49172 - 0.29268663035268940438e-1_f64 * t49176 + 0.34146773541147097178e-1_f64 * t49178 + 0.92526556154787137113e-2_f64 * t49186 - 0.46263278077393568556e-2_f64 * t49189 - 0.19514881078765566038e-1_f64 * t75089 + 0.13009920719177044025e-1_f64 * t75092 - 0.65854491829355115987e0_f64 * t820 * t22321 * t4057;
    t75097
}
