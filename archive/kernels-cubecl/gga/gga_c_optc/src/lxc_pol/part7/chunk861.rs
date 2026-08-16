//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 861/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk861<F: Float>(t1781: F, t287: F, t321: F, t320: F, t146: F, t318: F, t7192: F, t2737: F, t925: F, t2586: F, t2765: F, t940: F) -> (F, F, F, F, F, F) {
    let t8229 = t321 * t1781 * t287;
    let t8231 = F::cast_from(0.32196894406625029092e-1_f64) * t320 * t8229;
    let t8233 = t146 * t318 * t7192;
    let t8236 = t2737 * t925;
    let t8240 = t2586 * t2765;
    let t8241 = t940 * t8240;
    (t8229, t8231, t8233, t8236, t8240, t8241)
}
