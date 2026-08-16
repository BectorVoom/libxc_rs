//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2219/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2219<F: Float>(t28342: F, t81979: F, t17022: F, t1880: F, t1894: F, t214: F, t252: F, t5527: F, t25038: F, t6646: F, t829: F, t28333: F, t6562: F, t794: F) -> (F, F, F, F, F) {
    let t98330 = t81979 * t28342;
    let t98334 = t1880 * t214 * t1894 * t17022;
    let t98336 = t252 * t5527;
    let t98339 = t25038 * t6646 * t98336 * t829;
    let t98342 = t6562 * t794 * t28333;
    (t98330, t98334, t98336, t98339, t98342)
}
