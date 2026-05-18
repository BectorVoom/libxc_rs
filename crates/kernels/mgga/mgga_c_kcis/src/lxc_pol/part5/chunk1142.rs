//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1142/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1142<F: Float>(t14497: F, t18657: F, t330: F, t6539: F, t829: F, t2894: F, t14518: F, t14527: F, t14529: F, t19190: F, t19194: F, t19197: F, t19200: F, t19203: F, t9883: F, t991: F, t9918: F) -> F {
    let t19206 = t14497 * t18657;
    let t19209 = t6539 * t330;
    let t19210 = t19209 * t829;
    let t19211 = t2894 * t19210;
    let t19214 = t14518 - t14527 - t14529 / F::new(648.0) + t9883 - t9918 / F::new(1296.0) - t991 * t19190 / F::new(144.0) - t19194 / F::new(432.0) - t991 * t19197 / F::new(216.0) - t991 * t19200 / F::new(36.0) + F::new(7.0) / F::new(648.0) * t991 * t19203 + t991 * t19206 / F::new(54.0) - t991 * t19211 / F::new(288.0);
    t19214
}
