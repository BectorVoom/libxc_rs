//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1957/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1957<F: Float>(t23967: F, t26063: F, t7432: F, t84241: F, t2032: F, t22493: F, t24001: F, t26009: F, t26028: F, t26073: F, t32332: F, t7035: F, t7428: F, t7782: F, t84222: F, t84224: F, t84229: F, t84245: F, t90205: F, t9239: F) -> F {
    let t91921 = F::cast_from(80.0_f64) / F::cast_from(9.0_f64) * t23967 * t26063;
    let t91922 = t84241 * t7432;
    let t91938 = -F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t90205 * t2032 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t26073 * t7035 + t91921 - F::cast_from(440.0_f64) / F::cast_from(27.0_f64) * t91922 + F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t84245 * t7432 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t84222 - F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t84224 + F::cast_from(176.0_f64) / F::cast_from(27.0_f64) * t84229 - F::cast_from(40.0_f64) * t9239 * t32332 * t26009 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t26028 * t7035 + t7428 * t24001 / F::cast_from(3.0_f64) + t22493 * t7782 / F::cast_from(3.0_f64);
    t91938
}
