//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1409/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1409<F: Float>(t63332: F, t63334: F, t63888: F, t63893: F, t63911: F, t71142: F, t71144: F, t71146: F, t71152: F, t71154: F, t71156: F, t71408: F, t78002: F, t78005: F) -> F {
    let t78019 = F::cast_from(0.40256666666666666666e1_f64) * t78002 - F::cast_from(0.60384999999999999999e0_f64) * t78005 - F::cast_from(0.53675555555555555556e0_f64) * t63332 + F::cast_from(0.80513333333333333336e0_f64) * t63334 - F::cast_from(0.18396666666666666667e0_f64) * t63888 + F::cast_from(0.11038e1_f64) * t63893 + F::cast_from(0.80513333333333333333e0_f64) * t71142 - F::cast_from(0.24154e1_f64) * t71144 + F::cast_from(0.5519e0_f64) * t63911 - F::cast_from(0.22076e0_f64) * t71408 - F::cast_from(0.44729629629629629629e0_f64) * t71146 - F::cast_from(0.24154e1_f64) * t71152 - F::cast_from(0.40256666666666666668e0_f64) * t71154 + F::cast_from(0.16102666666666666667e1_f64) * t71156;
    t78019
}
