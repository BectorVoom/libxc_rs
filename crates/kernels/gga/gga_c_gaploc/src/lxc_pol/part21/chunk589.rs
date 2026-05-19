//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 589/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk589<F: Float>(t1382: F, t3366: F, t2877: F, t895: F, t2898: F, t901: F, t1645: F, t888: F) -> (F, F, F, F) {
    let t3368 = F::new(2.0) * t1382 * t3366;
    let t3370 = F::cast_from(0.35750489951850426669e0_f64) * t895 * t2877;
    let t3375 = t2898 * t901;
    let t3376 = F::cast_from(0.14896037479937677779e-1_f64) * t3375;
    let t3377 = t1645 * t888;
    (t3368, t3370, t3376, t3377)
}
