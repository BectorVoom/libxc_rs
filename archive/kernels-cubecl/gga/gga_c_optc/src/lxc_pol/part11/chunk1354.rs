//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1354/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1354<F: Float>(t33724: F, t33730: F, t43414: F, t44193: F, t44198: F, t58348: F, t58352: F, t58356: F, t58360: F, t58363: F, t58367: F, t11: F, t58354: F, t8620: F) -> (F, F) {
    let t58369 = -F::cast_from(0.27366666666666666666e-2_f64) * t44193 + F::cast_from(0.1642e-1_f64) * t44198 - F::cast_from(0.34468148148148148146e1_f64) * t43414 + F::cast_from(0.8042567901234567901e1_f64) * t33724 + F::cast_from(0.14595555555555555556e-1_f64) * t33730 + F::cast_from(0.15510666666666666667e2_f64) * t58348 + F::cast_from(0.14778e-1_f64) * t58352 - F::cast_from(0.3284e-2_f64) * t58356 - F::cast_from(0.44334e-1_f64) * t58360 + F::cast_from(0.9852e-2_f64) * t58363 - F::cast_from(0.1642e-2_f64) * t58367;
    let t58375 = t11 * t8620 * t58354;
    (t58369, t58375)
}
