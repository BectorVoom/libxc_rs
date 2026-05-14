//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 974/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk974<F: Float>(t2171: F, t7022: F, t2159: F, t22253: F, t22257: F, t22771: F, t22777: F, t22781: F, t22815: F, t22822: F, t22994: F, t23008: F, t23010: F, t23014: F, t23018: F, t23020: F, t3491: F, t3519: F, t686: F, t695: F, t696: F, t705: F, t9839: F) -> (F,) {
    let t23022 = t7022 * t2171;
    let t23024 = 0.13602790203758333267e0 * t2159 * t696 * t22253 + 0.3173984380876944429e0 * t2159 * t696 * t22781 + 0.20863587575493018851e1 * t686 * t3491 * t22815 + 0.2821319449668395048e0 * t22994 - 0.15114211337509259186e-1 * t695 * t696 * t22771 - 0.5441116081503333307e1 * t705 * t9839 * t22822 + 0.60456845350037036744e0 * t705 * t3519 * t22777 - 0.45342634012527777558e-1 * t695 * t696 * t22257 - 0.23981215322181357908e1 * t23008 - 0.48681704342817043984e1 * t23010 + 0.48681704342817043984e1 * t23014 + 0.10658317920969492404e2 * t23018 - 0.40568086952347536654e1 * t23020 - 0.23981215322181357908e1 * t23022;
    (t23024,)
}
