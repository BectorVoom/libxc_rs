//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1301/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1301<F: Float>(t10929: F, t10932: F, t2012: F, t10924: F, t2009: F, t6134: F, t1022: F, t7275: F, t2021: F, t10925: F, t5724: F, t1402: F, t2033: F, t3473: F) -> (F, F, F, F, F) {
    let t33356 = F::cast_from(0.55213813373645879534e2_f64) * t2012 * t10929 * t10932;
    let t33359 = F::cast_from(0.71500979903700853338e0_f64) * t6134 * t10924 * t2009;
    let t33360 = t7275 * t1022;
    let t33363 = F::cast_from(0.71500979903700853338e0_f64) * t2021 * t33360 * t2009;
    let t33365 = F::cast_from(0.35750489951850426669e0_f64) * t10925 * t5724;
    let t33367 = t2033 * t1402 * t3473;
    (t33356, t33359, t33363, t33365, t33367)
}
