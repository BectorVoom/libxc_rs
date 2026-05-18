//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 765/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk765<F: Float>(t1882: F, t3221: F, t11174: F, t443: F, t444: F, t3283: F, t1643: F, t3199: F, t8518: F, t1651: F, t3182: F, t1909: F) -> (F, F, F, F, F) {
    let t11999 = F::new(4.0) / F::new(9.0) * t1882 * t3221;
    let t12001 = t443 * t444 * t11174;
    let t12002 = t12001 * t3283;
    let t12004 = t3199 * t1643;
    let t12005 = t8518 * t12004;
    let t12008 = t3182 * t1651;
    let t12009 = t1909 * t12008;
    (t11999, t12001, t12002, t12005, t12009)
}
