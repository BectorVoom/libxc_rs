//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 440/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk440<F: Float>(t1469: F, t60: F, t1474: F, t1480: F, t44: F, t56: F, t61: F, t626: F) -> (F, F) {
    let t1483 = t60 * t1469;
    let t1486 = F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t44 * t1474 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t1480 * t61 - F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t56 * t1483 + t626;
    (t1483, t1486)
}
