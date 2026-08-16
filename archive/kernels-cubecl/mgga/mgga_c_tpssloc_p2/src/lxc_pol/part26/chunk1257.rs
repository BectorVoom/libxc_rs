//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1257/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1257<F: Float>(t22480: F, t4034: F, t22574: F, t55246: F, t8643: F, t23858: F, t6876: F, t26162: F, t55183: F, t6535: F, t9348: F, t12734: F) -> (F, F, F, F, F, F) {
    let t81412 = F::cast_from(6.0_f64) * t4034 * t22480;
    let t81419 = F::cast_from(9.0_f64) * t22574 * t8643 * t55246;
    let t81422 = F::cast_from(6.0_f64) * t6876 * t23858;
    let t81426 = F::cast_from(18.0_f64) * t22574 * t26162 * t55183;
    let t81430 = F::cast_from(6.0_f64) * t9348 * t6535;
    let t81432 = F::cast_from(12.0_f64) * t12734 * t6535;
    (t81412, t81419, t81422, t81426, t81430, t81432)
}
