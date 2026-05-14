//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 610/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk610<F: Float>(t194: F, t197: F, t8991: F, t815: F, t9636: F, t10: F, t296: F, t3050: F, t1636: F, t825: F, t89: F, t2404: F, t798: F, t2770: F, t863: F, t848: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10355 = t8991 / t197 / t194;
    let t10362 = t815 * t815;
    let t10363 = 1.0 / t10362;
    let t10373 = 0.18521666970164609055e-1 * t9636;
    let t10397 = t10 * t3050 * t296;
    let t10398 = 14.0 / 81.0 * t10397;
    let t10400 = t89 * t1636 * t825;
    let t10409 = t2404 * t798;
    let t10443 = t2770 * t863;
    let t10447 = t848 * t863;
    (t10355, t10363, t10373, t10397, t10398, t10400, t10409, t10443, t10447)
}
