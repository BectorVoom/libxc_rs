//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1686/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1686<F: Float>(t51978: F, t77505: F, t77507: F, t77509: F, t88104: F, t88108: F, t88114: F, t88118: F, t88122: F, t88126: F, t88130: F, t88134: F) -> F {
    let t88673 = F::cast_from(0.11111111111111111111e-1_f64) * t77505 - F::cast_from(0.24691358024691358025e-1_f64) * t88104 - F::cast_from(0.83333333333333333333e-2_f64) * t88108 - F::cast_from(0.44444444444444444444e-1_f64) * t77507 + F::cast_from(0.66666666666666666668e-1_f64) * t77509 + F::cast_from(0.11111111111111111111e0_f64) * t88114 - F::cast_from(0.55555555555555555555e-1_f64) * t88118 - F::cast_from(0.19999999999999999999e0_f64) * t88122 + F::cast_from(0.19999999999999999999e0_f64) * t88126 - F::cast_from(0.16666666666666666666e-1_f64) * t88130 - F::cast_from(0.22222222222222222222e-1_f64) * t88134 + F::cast_from(0.34567901234567901235e-1_f64) * t51978;
    t88673
}
