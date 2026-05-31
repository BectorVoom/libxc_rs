//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1645/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1645<F: Float>(t11300: F, t2926: F, t11299: F, t11132: F, t11134: F, t11136: F, t11138: F, t11140: F, t11147: F, t11153: F, t11158: F, t11162: F, t11167: F, t11171: F) -> (F, F, F, F) {
    let t11301 = t11300 * t2926;
    let t11303 = F::cast_from(0.96491876992155210402e2_f64) * t11299 * t11301;
    let t11304 = F::cast_from(28.0_f64) / F::cast_from(27.0_f64) * t11132;
    let t11315 = -t11304 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t11134 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t11136 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t11138 + t11140 / F::cast_from(3.0_f64) - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t11147 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t11153 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t11158 - F::cast_from(2.0_f64) * t11162 + F::cast_from(2.0_f64) * t11167 - t11171 / F::cast_from(3.0_f64);
    (t11301, t11303, t11304, t11315)
}
