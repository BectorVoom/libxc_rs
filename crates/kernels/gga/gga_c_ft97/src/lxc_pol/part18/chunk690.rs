//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 690/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk690<F: Float>(t3201: F, t8392: F, t1588: F, t920: F, t1903: F, t8217: F, t1647: F, t3199: F, t1902: F, t3170: F, t487: F) -> (F, F, F, F, F, F, F) {
    let t11826 = 2.0 / 27.0 * t8392 * t3201;
    let t11827 = t920 * t1588;
    let t11828 = t1903 * t11827;
    let t11829 = t8217 * t11828;
    let t11832 = t3199 * t1647;
    let t11833 = t1902 * t11832;
    let t11837 = t3170 * t487;
    (t11826, t11827, t11828, t11829, t11832, t11833, t11837)
}
