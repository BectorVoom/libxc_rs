//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3105/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3105<F: Float>(t58226: F, t68454: F, t68456: F, t68538: F, t68540: F, t68548: F, t68550: F, t68567: F, t68583: F, t68585: F, t68590: F, t81539: F) -> F {
    let t81766 = F::cast_from(0.69463333333333333333e-1_f64) * t81539 - F::cast_from(0.83356000000000000002e0_f64) * t68538 - F::cast_from(0.125034e1_f64) * t68540 + F::cast_from(0.13892666666666666667e0_f64) * t68548 + F::cast_from(0.41678000000000000001e0_f64) * t68550 - F::cast_from(0.20659e1_f64) * t68454 - F::cast_from(0.309885e1_f64) * t68456 - F::cast_from(0.20839e0_f64) * t68567 + t58226 + F::cast_from(0.34731666666666666667e0_f64) * t68583 + F::cast_from(0.69463333333333333335e0_f64) * t68585 - F::cast_from(0.11577222222222222223e0_f64) * t68590;
    t81766
}
