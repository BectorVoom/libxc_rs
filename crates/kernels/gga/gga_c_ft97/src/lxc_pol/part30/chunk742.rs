//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 742/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk742<F: Float>(t1403: F, t35251: F, t35255: F, t35259: F, t35263: F, t35267: F, t35270: F, t35276: F, t35282: F, t35287: F, t35297: F, t35302: F, t35304: F, t6002: F, t6745: F, t6754: F, t6840: F, t6844: F, t7437: F, t7443: F, t7487: F, t7491: F) -> (F,) {
    let t35306 = -t6002 * t35251 / 18.0 + t6002 * t35255 / 9.0 - t6002 * t35259 / 9.0 + t1403 * t35263 / 3.0 + t1403 * t35267 - 2.0 / 3.0 * t1403 * t35270 - t7437 * t6754 / 3.0 + t1403 * t35276 / 3.0 + t6745 * t7491 / 3.0 + t1403 * t35282 / 6.0 - 2.0 / 3.0 * t1403 * t35287 - t6745 * t7443 / 3.0 + t7437 * t6844 / 6.0 + t7437 * t6840 / 6.0 - t1403 * t35297 / 3.0 + t6745 * t7487 / 6.0 - 4.0 * t35302 - 4.0 * t35304;
    (t35306,)
}
