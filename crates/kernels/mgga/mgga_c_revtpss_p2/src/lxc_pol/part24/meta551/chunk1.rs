//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1639/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1639<F: Float>(t87302: F, t87316: F, t87931: F, t87942: F, t87951: F, t87952: F, t87966: F, t87987: F, t6206: F, t6226: F, t981: F, t19133: F, t19303: F) -> (F, F, F) {
    let t87990 = t87302 + t87316 + t87931 + t87942 + t87951 + t87952 + t87966 + t87987;
    let t88004 = F::cast_from(0.21053605041484726346e2_f64) * t981 * t6226 * t6206;
    let t88007 = F::cast_from(0.62337092780453269531e3_f64) * t981 * t19133 * t19303;
    (t87990, t88004, t88007)
}
