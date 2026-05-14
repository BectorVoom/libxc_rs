//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1096/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1096<F: Float>(t13580: F, t24389: F, t3750: F, t6767: F, t24361: F, t24378: F, t27647: F, t27671: F, t420: F, t55105: F, t2413: F, t27659: F, t27660: F, t1113: F, t2440: F, t226: F) -> (F, F, F, F, F, F, F) {
    let t108845 = t13580 * t24389 * t3750;
    let t108848 = t13580 * t6767;
    let t108857 = t24361 * t24378 * t27647;
    let t108860 = t27671 * t420 * t55105;
    let t108871 = t27659 * t27660 * t2413;
    let t108874 = t2440 * t1113;
    let t108880 = t1113 * t226;
    (t108845, t108848, t108857, t108860, t108871, t108874, t108880)
}
