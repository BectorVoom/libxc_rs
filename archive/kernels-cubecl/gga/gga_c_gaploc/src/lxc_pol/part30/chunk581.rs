//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 581/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk581<F: Float>(t3295: F, t969: F, t825: F, t3191: F, t325: F, t3190: F, t813: F, t2685: F, t2684: F, t894: F, t988: F) -> (F, F, F, F, F, F, F, F) {
    let t3296 = t969 * t3295;
    let t3297 = t825 * t3296;
    let t3298 = F::cast_from(0.38342925953920749676e0_f64) * t3297;
    let t3307 = t3191 * t325;
    let t3308 = t3190 * t3307;
    let t3309 = t813 * t3308;
    let t3311 = t2685 * t3295;
    let t3312 = t2684 * t3311;
    let t3313 = F::cast_from(0.38342925953920749676e0_f64) * t3312;
    let t3327 = t894 * t988;
    (t3296, t3298, t3307, t3308, t3309, t3311, t3313, t3327)
}
