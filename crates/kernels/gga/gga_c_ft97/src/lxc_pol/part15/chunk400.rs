//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 400/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk400<F: Float>(t1882: F, t951: F, t1546: F, t89: F, t998: F, t1018: F, t375: F, t1017: F, t1984: F, t1008: F, t549: F, t2007: F, t929: F) -> (F, F, F, F, F, F) {
    let t3286 = t1882 * t951;
    let t3318 = t89 * t1546 * t998;
    let t3335 = t89 * t375 * t1018;
    let t3342 = t1984 * t1017;
    let t3355 = t549 * t1008;
    let t3359 = t2007 * t929;
    (t3286, t3318, t3335, t3342, t3355, t3359)
}
