//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 499/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk499<F: Float>(t103: F, t1449: F, t100: F, t104: F, t1445: F, t1447: F, t92: F) -> (F, F) {
    let t1450 = t103 * t1449;
    let t1453 = F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t100 * t1450 - F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t1447 * t104 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t92 * t1445;
    (t1450, t1453)
}
