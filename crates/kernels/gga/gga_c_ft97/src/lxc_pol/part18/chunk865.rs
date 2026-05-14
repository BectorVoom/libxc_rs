//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 865/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk865<F: Float>(t1876: F, t23339: F, t11810: F, t5632: F, t8392: F, t5631: F, t8372: F, t1825: F, t452: F, t5722: F, t432: F, t5743: F, t488: F, t1882: F, t5712: F, t5661: F) -> (F, F, F, F, F, F, F, F, F) {
    let t23340 = t23339 * t1876;
    let t23341 = t11810 * t23340;
    let t23344 = t8392 * t5632;
    let t23346 = t8372 * t5631;
    let t23350 = t452 * t1825 * t5722;
    let t23353 = t5743 * t432;
    let t23355 = t452 * t488 * t23353;
    let t23358 = t1882 * t5712;
    let t23360 = t1882 * t5661;
    (t23340, t23341, t23344, t23346, t23350, t23353, t23355, t23358, t23360)
}
