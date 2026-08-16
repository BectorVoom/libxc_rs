//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1059/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1059(t23017: f64, t697: f64, t2136: f64, t7030: f64, t2171: f64, t7022: f64, t2159: f64, t22253: f64, t22257: f64, t22771: f64, t22777: f64, t22781: f64, t22815: f64, t22822: f64, t22994: f64, t23008: f64, t23010: f64, t23014: f64, t3491: f64, t3519: f64, t686: f64, t695: f64, t696: f64, t705: f64, t9839: f64) -> f64 {
    let t23018 = t23017 * t697;
    let t23020 = t7030 * t2136;
    let t23022 = t7022 * t2171;
    let t23024 = 0.13602790203758333267e0_f64 * t2159 * t696 * t22253 + 0.3173984380876944429e0_f64 * t2159 * t696 * t22781 + 0.20863587575493018851e1_f64 * t686 * t3491 * t22815 + 0.2821319449668395048e0_f64 * t22994 - 0.15114211337509259186e-1_f64 * t695 * t696 * t22771 - 0.5441116081503333307e1_f64 * t705 * t9839 * t22822 + 0.60456845350037036744e0_f64 * t705 * t3519 * t22777 - 0.45342634012527777558e-1_f64 * t695 * t696 * t22257 - 0.23981215322181357908e1_f64 * t23008 - 0.48681704342817043984e1_f64 * t23010 + 0.48681704342817043984e1_f64 * t23014 + 0.10658317920969492404e2_f64 * t23018 - 0.40568086952347536654e1_f64 * t23020 - 0.23981215322181357908e1_f64 * t23022;
    t23024
}
