//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1016/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1016<F: Float>(t41951: F, t80770: F, t80772: F, t80819: F, t80821: F, t88221: F, t88225: F, t88229: F, t89027: F, t89030: F, t89034: F, t89038: F, t89042: F, t89047: F, t81095: F, t81102: F, t81105: F, t81124: F, t81131: F, t89051: F, t89054: F, t89058: F, t89062: F, t89069: F, t89073: F, t89077: F, t89081: F, t89085: F) -> (F, F) {
    let t89727 = 4.0 / 27.0 * t80770 - 4.0 / 27.0 * t80772 + t41951 + 2.0 / 9.0 * t88221 - 2.0 / 3.0 * t88225 - 4.0 / 9.0 * t88229 - 2.0 / 9.0 * t80819 - 2.0 / 9.0 * t80821 - 10.0 / 27.0 * t89027 - 2.0 * t89030 - 4.0 / 3.0 * t89034 - t89038 / 9.0 + t89042 / 3.0 - 40.0 / 243.0 * t89047;
    let t89741 = -t89051 / 6.0 + t89054 + 4.0 * t89058 - t89062 / 18.0 + 4.0 / 9.0 * t81095 - 4.0 / 3.0 * t81102 + 2.0 / 27.0 * t81105 - 6.0 * t89069 + 20.0 / 27.0 * t89073 + 4.0 / 3.0 * t89077 + 4.0 / 3.0 * t89081 - 4.0 / 3.0 * t89085 + 2.0 / 9.0 * t81124 + 20.0 / 243.0 * t81131;
    (t89727, t89741)
}
