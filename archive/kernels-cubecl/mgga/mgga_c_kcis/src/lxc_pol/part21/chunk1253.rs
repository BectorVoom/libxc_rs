//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1253/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1253<F: Float>(t26656: F, t27744: F, t26395: F, t26397: F, t26405: F, t26408: F, t26660: F, t28901: F, t91769: F, t91772: F, t91773: F, t91776: F, t91777: F, t91778: F, t91781: F, t95270: F, t95271: F, t95272: F, t95273: F, t95274: F) -> F {
    let t95275 = F::cast_from(4.0_f64) * t26656;
    let t95276 = t27744 / F::cast_from(8.0_f64);
    let t95277 = t95270 - t91769 - t26395 - t26397 + t91772 + t91773 - t26405 - t26408 + t95271 - t91776 - t95272 + t95273 + t91777 + t95274 + t28901 - t91778 + t95275 - t95276 - t26660 - t91781;
    t95277
}
