//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 1033/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk1033<F: Float>(t31304: F, t6997: F, t649: F, t6534: F, t2040: F, t31537: F, t7050: F, t22607: F, t8644: F, t1983: F, t24166: F, t8643: F) -> (F, F, F, F, F, F) {
    let t115721 = F::cast_from(2.0_f64) * t31304 * t6997;
    let t115723 = t649 * t6534;
    let t115725 = F::cast_from(4.0_f64) * t115723 * t2040;
    let t115727 = F::cast_from(4.0_f64) * t31537 * t7050;
    let t115728 = t22607 * t8644;
    let t115732 = t1983 * t24166 * t8643;
    (t115721, t115723, t115725, t115727, t115728, t115732)
}
