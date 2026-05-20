//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1135/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1135<F: Float>(t11134: F, t11136: F, t11138: F, t11140: F, t11147: F, t11153: F, t11158: F, t11162: F, t11167: F, t11171: F, t11890: F, t341: F) -> (F, F) {
    let t11901 = -t11890 - F::cast_from(0.11111111111111111111e-1_f64) * t11134 + F::cast_from(0.55555555555555555555e-2_f64) * t11136 - F::cast_from(0.16666666666666666667e-1_f64) * t11138 + F::cast_from(0.83333333333333333334e-2_f64) * t11140 - F::cast_from(0.92592592592592592592e-2_f64) * t11147 + F::cast_from(0.33333333333333333333e-1_f64) * t11153 - F::cast_from(0.16666666666666666666e-1_f64) * t11158 - F::cast_from(0.50000000000000000001e-1_f64) * t11162 + F::cast_from(0.50000000000000000001e-1_f64) * t11167 - F::cast_from(0.83333333333333333333e-2_f64) * t11171;
    let t11902 = t11901 * t341;
    (t11901, t11902)
}
