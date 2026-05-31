//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 710/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk710<F: Float>(t11200: F, t26: F, t2999: F, t11167: F, t11170: F, t11172: F, t11177: F, t11180: F, t11183: F, t11186: F, t11189: F, t11192: F, t11195: F, t11198: F, t7945: F, t7946: F, t7948: F, t7950: F, t7952: F) -> (F, F) {
    let t11202 = t26 * t2999 * t11200;
    let t11204 = -t7945 - F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t7946 + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t7948 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t7950 + t7952 / F::cast_from(9.0_f64) - F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t11167 + t11170 - t11172 + F::cast_from(22.0_f64) / F::cast_from(9.0_f64) * t11177 - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t11180 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t11183 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t11186 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t11189 - F::cast_from(2.0_f64) * t11192 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t11195 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t11198 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t11202;
    (t11202, t11204)
}
