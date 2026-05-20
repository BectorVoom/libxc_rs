//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1070/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1070<F: Float>(t11133: F, t11134: F, t11136: F, t11138: F, t11140: F, t11147: F, t11153: F, t11158: F, t11162: F, t11167: F, t11171: F) -> F {
    let t11173 = -t11133 - F::cast_from(0.19755555555555555556e-1_f64) * t11134 + F::cast_from(0.9877777777777777778e-2_f64) * t11136 - F::cast_from(0.29633333333333333334e-1_f64) * t11138 + F::cast_from(0.14816666666666666667e-1_f64) * t11140 - F::cast_from(0.16462962962962962963e-1_f64) * t11147 + F::cast_from(0.59266666666666666668e-1_f64) * t11153 - F::cast_from(0.29633333333333333334e-1_f64) * t11158 - F::cast_from(0.88900000000000000002e-1_f64) * t11162 + F::cast_from(0.88900000000000000002e-1_f64) * t11167 - F::cast_from(0.14816666666666666667e-1_f64) * t11171;
    t11173
}
