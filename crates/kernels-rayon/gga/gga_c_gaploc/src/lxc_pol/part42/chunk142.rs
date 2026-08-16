//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 142/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk142(t255: f64, t256: f64, t64: f64, t1: f64, t252: f64, t341: f64, t345: f64, t347: f64, t14: f64, t344: f64, t337: f64, t359: f64, t642: f64, t645: f64, t648: f64) -> (f64, f64, f64) {
    let t656 = 1.0_f64 / t256 / t255;
    let t657 = t64 * t656;
    let t659 = t341 * t252 * t1;
    let t664 = -0.14921166666666666667e-3_f64 * t345 - 0.39332083333333333333e-2_f64 * t347;
    let t667 = -t659 * t344 / 12.0_f64 + t14 * t664 / 2.0_f64;
    let t670 = t337 + t359 + t642 - t645 - t648;
    (t657, t667, t670)
}
