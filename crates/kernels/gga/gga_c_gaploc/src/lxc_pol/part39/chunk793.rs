//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 793/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk793<F: Float>(t12653: F, t12223: F, t935: F, t1445: F, t813: F, t12213: F) -> (F, F, F, F, F) {
    let t13855 = F::cast_from(0.38342925953920749677e0_f64) * t12653;
    let t13857 = t12223 * t935;
    let t13858 = t1445 * t13857;
    let t13859 = t813 * t13858;
    let t13861 = t12213 * t935;
    (t13855, t13857, t13858, t13859, t13861)
}
