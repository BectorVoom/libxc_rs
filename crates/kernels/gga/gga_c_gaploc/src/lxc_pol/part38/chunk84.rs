//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 84/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk84<F: Float>(t11: F, t1: F, t344: F, t21: F, t86: F, t345: F, t347: F, t30: F, t340: F) -> (F, F, F) {
    let t349 = F::sqrt(t11);
    let t350 = t349 * t1;
    let t351 = t350 * t344;
    let t353 = t21 * t86;
    let t355 = -F::new(0.632975e0) * t345 - F::cast_from(0.29896666666666666667e0_f64) * t347 - F::new(0.1023875e0) * t351 - F::cast_from(0.82156666666666666667e-1_f64) * t353;
    let t356 = F::new(1.0) / t30;
    let t357 = t355 * t356;
    let t359 = F::new(1.0) * t340 * t357;
    (t351, t353, t359)
}
