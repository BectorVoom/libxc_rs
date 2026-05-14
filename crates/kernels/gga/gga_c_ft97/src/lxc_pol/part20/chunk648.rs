//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 648/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk648<F: Float>(t258: F, t3821: F, t684: F, t2599: F, t1160: F, t2492: F, t2602: F, t265: F, t9895: F) -> (F, F, F, F, F) {
    let t14154 = t258 * t3821;
    let t14155 = t14154 * t684;
    let t14156 = t2599 * t14155;
    let t14159 = t2492 * t1160;
    let t14160 = t14159 * t2602;
    let t14163 = t9895 * t265;
    (t14155, t14156, t14159, t14160, t14163)
}
