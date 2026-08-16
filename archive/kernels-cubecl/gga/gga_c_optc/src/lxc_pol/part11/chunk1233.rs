//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1233/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1233<F: Float>(t56275: F, t56287: F, t59: F, t85: F, t22497: F, t22562: F, t22578: F, t22657: F, t22659: F, t22661: F, t22666: F, t22675: F, t22694: F, t56062: F, t56068: F, t56263: F) -> (F, F, F) {
    let t56289 = (t56275 + t56287) * t59;
    let t56291 = F::cast_from(0.19751789702565206229e-1_f64) * t56289 * t85;
    let t56292 = -t22657 + t56062 - t22659 - t22661 - t56068 + t22666 + t56263 - t22675 - t22694 + t56291 - t22497 + t22562 + t22578;
    (t56289, t56291, t56292)
}
