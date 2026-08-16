//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 742/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk742<F: Float>(t11021: F, t11023: F, t11025: F, t11019: F, t11036: F, t7775: F, t7778: F, t7782: F, t7820: F, t8192: F, t8195: F, t11043: F) -> (F, F) {
    let t11646 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t11021;
    let t11647 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t11023;
    let t11648 = F::cast_from(4.0_f64) / F::cast_from(81.0_f64) * t11025;
    let t11656 = t11019 / F::cast_from(9.0_f64) - t11646 - t11647 + t11648 - F::cast_from(8.0_f64) / F::cast_from(81.0_f64) * t7775 + t7778 / F::cast_from(27.0_f64) + F::cast_from(2.0_f64) / F::cast_from(81.0_f64) * t7782 - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t7820 - F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t8192 + t8195 / F::cast_from(9.0_f64) - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t11036;
    let t11659 = F::cast_from(4.0_f64) / F::cast_from(81.0_f64) * t11043;
    (t11656, t11659)
}
