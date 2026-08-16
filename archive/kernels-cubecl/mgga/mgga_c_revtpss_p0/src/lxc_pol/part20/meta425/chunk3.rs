//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1596/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1596<F: Float>(t43762: F, t43769: F, t43771: F, t43773: F, t43779: F, t43781: F, t43783: F, t43785: F, t43787: F, t43791: F, t43795: F, t43799: F, t43802: F, t43804: F) -> F {
    let t44036 = -F::cast_from(0.97370864197530864196e-1_f64) * t43762 - F::cast_from(0.85199506172839506175e-1_f64) * t43769 - F::cast_from(0.97370864197530864199e0_f64) * t43771 + F::cast_from(0.43816888888888888888e0_f64) * t43773 + F::cast_from(0.43816888888888888889e0_f64) * t43779 + F::cast_from(0.54771111111111111111e0_f64) * t43781 + F::cast_from(0.10954222222222222222e1_f64) * t43783 - F::cast_from(0.21908444444444444444e0_f64) * t43785 - F::cast_from(0.13145066666666666666e1_f64) * t43787 - F::cast_from(0.98587999999999999998e0_f64) * t43791 + F::cast_from(0.197176e1_f64) * t43795 + F::cast_from(0.82156666666666666667e-1_f64) * t43799 + F::cast_from(0.85451625e1_f64) * t43802 - F::cast_from(0.379785e1_f64) * t43804;
    t44036
}
