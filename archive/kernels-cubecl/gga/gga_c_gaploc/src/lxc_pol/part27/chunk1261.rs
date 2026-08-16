//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1261/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1261<F: Float>(t33337: F, t2536: F, t2925: F, t2009: F, t2021: F, t10821: F, t23157: F, t10929: F, t10932: F, t2012: F, t10924: F, t6134: F) -> (F, F, F, F, F) {
    let t33338 = F::cast_from(0.76685851907841499352e0_f64) * t33337;
    let t33348 = t2536 * t2925;
    let t33351 = F::cast_from(0.71500979903700853338e0_f64) * t2021 * t33348 * t2009;
    let t33353 = F::cast_from(0.12423108009070322895e3_f64) * t23157 * t10821;
    let t33356 = F::cast_from(0.55213813373645879534e2_f64) * t2012 * t10929 * t10932;
    let t33359 = F::cast_from(0.71500979903700853338e0_f64) * t6134 * t10924 * t2009;
    (t33338, t33351, t33353, t33356, t33359)
}
