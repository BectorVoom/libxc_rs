//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 503/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk503<F: Float>(t191: F, t7640: F, t815: F, t287: F, t2404: F, t798: F, t2770: F, t863: F, t848: F, t2344: F, t2680: F, t665: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10261 = t191 * t7640;
    let t10362 = t815 * t815;
    let t10363 = F::new(1.0) / t10362;
    let t10364 = t287 * t10363;
    let t10409 = t2404 * t798;
    let t10443 = t2770 * t863;
    let t10447 = t848 * t863;
    let t10478 = t2344 * t798;
    let t10491 = t665 * t2680;
    (t10261, t10362, t10363, t10364, t10409, t10443, t10447, t10478, t10491)
}
