//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 995/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk995<F: Float>(t1882: F, t33157: F, t33031: F, t8392: F, t33028: F, t33009: F, t33062: F, t33138: F, t33082: F, t33017: F, t7402: F, t8232: F) -> (F, F, F, F, F, F, F, F, F) {
    let t140161 = t1882 * t33157;
    let t140169 = t8392 * t33031;
    let t140237 = t1882 * t33028;
    let t140239 = t1882 * t33009;
    let t140241 = t1882 * t33062;
    let t140253 = t1882 * t33138;
    let t140263 = t1882 * t33082;
    let t140268 = t1882 * t33017;
    let t140274 = F::new(8.0) / F::new(27.0) * t8232 * t7402;
    (t140161, t140169, t140237, t140239, t140241, t140253, t140263, t140268, t140274)
}
