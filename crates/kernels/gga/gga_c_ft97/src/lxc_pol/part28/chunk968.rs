//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 968/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk968<F: Float>(t1882: F, t32483: F, t32617: F, t32573: F, t32524: F, t7231: F, t8232: F, t32559: F, t32564: F, t32551: F, t32591: F, t32632: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t138168 = t1882 * t32483;
    let t138176 = t1882 * t32617;
    let t138178 = t1882 * t32573;
    let t138184 = t1882 * t32524;
    let t138208 = F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t8232 * t7231;
    let t138221 = t1882 * t32559;
    let t138223 = t1882 * t32564;
    let t138254 = t1882 * t32551;
    let t138283 = t1882 * t32591;
    let t138285 = t1882 * t32632;
    (t138168, t138176, t138178, t138184, t138208, t138221, t138223, t138254, t138283, t138285)
}
