//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1249/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1249<F: Float>(t4727: F, t22497: F, t22562: F, t22578: F, t22581: F, t22593: F, t22697: F, t22703: F, t22708: F, t22711: F, t48162: F, t56294: F) -> (F, F) {
    let t56654 = t4727 * t4727;
    let t56661 = -t22497 + t22562 + t22578 + t22581 - t22593 - t56294 - F::cast_from(14.0_f64) * t48162 + t22697 + t22703 + t22708 - t22711;
    (t56654, t56661)
}
