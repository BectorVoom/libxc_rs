//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 832/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk832<F: Float>(t22241: F, t2874: F, t10503: F, t22186: F, t2881: F, t1248: F, t5309: F, t10697: F, t296: F, t21355: F, t2857: F, t319: F) -> (F, F, F, F, F, F, F) {
    let t22242 = t2874 * t22241;
    let t22245 = t10503 * t22186;
    let t22246 = t2881 * t22245;
    let t22249 = t5309 * t1248;
    let t22250 = t10697 * t22249;
    let t22251 = t296 * t22250;
    let t22255 = t2857 * t319 * t21355;
    (t22242, t22245, t22246, t22249, t22250, t22251, t22255)
}
