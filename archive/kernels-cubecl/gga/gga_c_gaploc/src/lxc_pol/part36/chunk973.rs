//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 973/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk973<F: Float>(t33348: F, t787: F, t9824: F, t10892: F, t2021: F, t7372: F, t13042: F, t2197: F, t8793: F, t9950: F, t3040: F, t41236: F) -> (F, F, F, F, F) {
    let t43526 = t787 * t33348 * t9824;
    let t43527 = F::cast_from(0.29792074959875355558e-1_f64) * t43526;
    let t43529 = t2021 * t10892 * t7372;
    let t43567 = F::cast_from(0.43710935587469654631e2_f64) * t2197 * t13042;
    let t43569 = F::cast_from(0.10725146985555128001e1_f64) * t8793 * t9950;
    let t43571 = F::cast_from(0.35750489951850426669e0_f64) * t41236 * t3040;
    (t43527, t43529, t43567, t43569, t43571)
}
