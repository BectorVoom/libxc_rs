//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1251/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1251<F: Float>(t10755: F, t1987: F, t10833: F, t17637: F, t730: F, t9532: F, t2860: F, t9533: F, t9356: F, t10829: F, t1954: F, t723: F) -> (F, F, F, F, F) {
    let t30710 = F::cast_from(0.35089341735807877242e1_f64) * t1987 * t10755;
    let t30714 = F::cast_from(0.12304822629859687989e5_f64) * t730 * t17637 * t10833 * t9532;
    let t30716 = F::cast_from(0.30762056574649219972e4_f64) * t2860 * t9533;
    let t30718 = F::cast_from(0.35089341735807877242e1_f64) * t2860 * t9356;
    let t30722 = F::cast_from(0.11696447245269292414e1_f64) * t730 * t1954 * t10829 * t723;
    (t30710, t30714, t30716, t30718, t30722)
}
