//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 757/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk757<F: Float>(t342: F, t630: F, t7430: F, t231: F, t6061: F, t1403: F, t1526: F, t2: F, t2320: F, t33540: F, t33545: F, t33547: F, t33552: F, t343: F, t6136: F, t6141: F, t7426: F, t7427: F) -> (F, F, F) {
    let t33557 = t342 * t630 * t7430 / F::cast_from(12.0_f64);
    let t33561 = t231 * t6061;
    let t33566 = (-t33540 * t7427 / F::cast_from(6.0_f64) + t33545 + t1403 * t33547 / F::cast_from(18.0_f64) + t1403 * t6141 / F::cast_from(3.0_f64) - t7426 * t33552 / F::cast_from(6.0_f64) - t33557 - t1526 * t2320 * t6136 / F::cast_from(12.0_f64) - t342 * t343 * t33561 / F::cast_from(4.0_f64)) * t2;
    (t33557, t33561, t33566)
}
