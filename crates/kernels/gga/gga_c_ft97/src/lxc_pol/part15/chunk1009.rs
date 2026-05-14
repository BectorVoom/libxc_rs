//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1009/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1009<F: Float>(t42044: F, t80819: F, t80821: F, t88218: F, t88221: F, t88225: F, t88229: F, t88233: F, t88237: F, t89022: F, t89027: F, t89030: F, t89034: F, t89038: F, t89042: F, t81095: F, t81102: F, t81105: F, t81124: F, t81131: F, t89047: F, t89051: F, t89054: F, t89058: F, t89062: F, t89069: F, t89073: F, t89077: F, t89081: F, t89085: F) -> (F, F) {
    let t89529 = -t88218 / 3.0 + t42044 + 4.0 / 9.0 * t88221 - 4.0 / 3.0 * t88225 - 8.0 / 9.0 * t88229 - t88233 / 4.0 - 5.0 / 16.0 * t88237 + t89022 / 6.0 - 4.0 / 9.0 * t80819 - 4.0 / 9.0 * t80821 - 20.0 / 27.0 * t89027 - 4.0 * t89030 - 8.0 / 3.0 * t89034 - 2.0 / 9.0 * t89038 + 2.0 / 3.0 * t89042;
    let t89545 = -80.0 / 243.0 * t89047 - t89051 / 3.0 + 2.0 * t89054 + 8.0 * t89058 - t89062 / 9.0 + 8.0 / 9.0 * t81095 - 8.0 / 3.0 * t81102 + 4.0 / 27.0 * t81105 - 12.0 * t89069 + 40.0 / 27.0 * t89073 + 8.0 / 3.0 * t89077 + 8.0 / 3.0 * t89081 - 8.0 / 3.0 * t89085 + 4.0 / 9.0 * t81124 + 40.0 / 243.0 * t81131;
    (t89529, t89545)
}
