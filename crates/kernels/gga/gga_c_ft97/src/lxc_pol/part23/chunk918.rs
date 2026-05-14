//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 918/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk918<F: Float>(t1424: F, t3972: F, t729: F, t762: F, t1882: F, t6871: F, t13839: F, t6162: F, t24668: F, t3859: F, t14127: F, t191: F, t241: F, t255: F) -> (F, F, F, F, F, F, F, F) {
    let t28284 = t1424 * t3972;
    let t28286 = t729 * t762 * t28284;
    let t28289 = t1882 * t6871;
    let t28291 = t13839 * t6162;
    let t28294 = t24668 * t3859;
    let t28295 = t14127 * t28294;
    let t28298 = t191 * t241;
    let t28299 = t28298 * t255;
    (t28284, t28286, t28289, t28291, t28294, t28295, t28298, t28299)
}
