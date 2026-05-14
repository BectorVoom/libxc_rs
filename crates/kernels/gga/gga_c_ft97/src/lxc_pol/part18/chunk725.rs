//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 725/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk725<F: Float>(t12448: F, t1995: F, t527: F, t12444: F, t133: F, t1595: F, t929: F, t120: F, t378: F, t11088: F, t72: F, t422: F, t383: F, t8966: F, t11145: F, t71: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12449 = t1995 * t12448;
    let t12452 = t527 * t12448;
    let t12455 = t133 * t12444;
    let t12462 = t929 * t1595;
    let t12464 = t378 * t12462 * t120;
    let t12471 = t11088 * t120;
    let t12472 = t72 * t12471;
    let t12477 = t422 * t929;
    let t12478 = t12477 * t383;
    let t12479 = t12478 * t8966;
    let t12483 = t72 * t11145 * t120;
    let t12486 = t71 * t929;
    (t12449, t12452, t12455, t12464, t12472, t12477, t12479, t12483, t12486)
}
