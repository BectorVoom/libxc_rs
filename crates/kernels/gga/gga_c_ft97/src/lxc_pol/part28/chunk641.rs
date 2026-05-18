//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 641/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk641<F: Float>(t26050: F, t83: F, t1882: F, t6531: F, t6540: F, t3214: F, t452: F, t5710: F, t11593: F, t1901: F, t23152: F, t26185: F, t26189: F, t26192: F, t26195: F, t26199: F, t26203: F, t26207: F, t26211: F, t26214: F, t446: F) -> F {
    let t26217 = t83 * t26050;
    let t26220 = t1882 * t6531;
    let t26222 = t1882 * t6540;
    let t26225 = t452 * t5710 * t3214;
    let t26228 = t1901 * t26185 / F::new(9.0) + F::new(2.0) / F::new(9.0) * t11593 * t26189 + t26192 / F::new(9.0) - t446 * t26195 / F::new(3.0) - t1901 * t26199 / F::new(9.0) - F::new(2.0) / F::new(9.0) * t1901 * t26203 - t446 * t26207 / F::new(9.0) + t23152 + F::new(2.0) / F::new(9.0) * t11593 * t26211 + t1901 * t26214 / F::new(9.0) - t446 * t26217 / F::new(3.0) + t26220 / F::new(9.0) - t26222 / F::new(9.0) + t446 * t26225 / F::new(3.0);
    t26228
}
