//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1078/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1078<F: Float>(t27512: F, t27515: F, t27518: F, t27522: F, t27524: F, t27527: F, t27530: F, t27533: F, t27535: F, t27537: F, t27539: F, t27541: F, t27545: F, t27547: F, t27549: F, t27551: F) -> F {
    let t27693 = F::cast_from(0.9375e-1_f64) * t27512 - F::cast_from(0.1875e0_f64) * t27515 + F::cast_from(0.125e0_f64) * t27518 + F::cast_from(0.1875e0_f64) * t27522 - F::cast_from(0.125e0_f64) * t27524 - F::cast_from(0.9375e-1_f64) * t27527 - F::cast_from(0.20833333333333333333e-1_f64) * t27530 + F::cast_from(0.625e-1_f64) * t27533 - F::cast_from(0.20234375e-1_f64) * t27535 + F::cast_from(0.4046875e-1_f64) * t27537 - F::cast_from(0.53958333333333333334e-1_f64) * t27539 - F::cast_from(0.4046875e-1_f64) * t27541 + F::cast_from(0.53958333333333333334e-1_f64) * t27545 + F::cast_from(0.20234375e-1_f64) * t27547 - F::cast_from(0.89930555555555555557e-2_f64) * t27549 - F::cast_from(0.26979166666666666667e-1_f64) * t27551;
    t27693
}
