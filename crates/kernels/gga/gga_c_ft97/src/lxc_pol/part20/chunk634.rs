//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 634/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk634<F: Float>(t13672: F, t265: F, t729: F, t1131: F, t2619: F, t2526: F, t762: F, t1160: F, t2567: F) -> (F, F, F, F, F) {
    let t13915 = t729 * t265 * t13672;
    let t13919 = t729 * t2619 * t1131;
    let t13922 = t1131 * t2526;
    let t13924 = t729 * t762 * t13922;
    let t13927 = t1160 * t2567;
    (t13915, t13919, t13922, t13924, t13927)
}
