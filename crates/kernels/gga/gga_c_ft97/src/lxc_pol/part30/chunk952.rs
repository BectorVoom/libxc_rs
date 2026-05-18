//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 952/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk952<F: Float>(t1882: F, t33651: F, t7499: F, t8232: F, t33756: F, t8392: F, t33725: F, t33654: F, t10051: F, t7546: F, t33748: F, t2567: F, t7553: F) -> (F, F, F, F, F, F, F, F) {
    let t141817 = t1882 * t33651;
    let t141820 = F::new(8.0) / F::new(27.0) * t8232 * t7499;
    let t141834 = t8392 * t33756;
    let t141850 = t1882 * t33725;
    let t141852 = t1882 * t33654;
    let t141868 = t10051 * t7546;
    let t141873 = t1882 * t33748;
    let t141882 = t2567 * t7553;
    (t141817, t141820, t141834, t141850, t141852, t141868, t141873, t141882)
}
