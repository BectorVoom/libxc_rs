//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2151/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2151<F: Float>(t1912: F, t46452: F, t82143: F, t82145: F, t82150: F, t855: F, t858: F, t87029: F, t87033: F, t87039: F, t87042: F, t87047: F, t87050: F, t87094: F, t87146: F, t87524: F, t87562: F, t87606: F, t87656: F, t87694: F, t87735: F) -> F {
    let t87741 = t87029 + F::cast_from(0.19190897446562641759e-1_f64) * t82143 - F::cast_from(0.16449340668482264365e-1_f64) * t87033 - t46452 * t1912 - F::cast_from(0.6579736267392905746e-1_f64) * t87039 + F::cast_from(0.38381794893125283518e-1_f64) * t82145 - t87042 + F::cast_from(0.38381794893125283518e-1_f64) * t82150 + F::cast_from(0.82246703342411321825e-2_f64) * t87047 - F::cast_from(0.2302907693587517011e0_f64) * t87050 - t855 * t858 * (t87094 + t87146 + t87524 + t87562 + t87606 + t87656 + t87694 + t87735);
    t87741
}
