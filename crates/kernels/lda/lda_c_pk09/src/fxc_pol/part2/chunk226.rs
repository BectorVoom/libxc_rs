//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 226/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk226<F: Float>(t200: F, t573: F, t205: F, t569: F, t179: F, t192: F, t127: F, t129: F, t126: F) -> (F, F, F, F, F, F) {
    let t882 = F::cast_from(1.6183441301295518_f64) * t200 * t573;
    let t884 = F::cast_from(1.5795946107075225_f64) * t205 * t569;
    let t886 = F::cast_from(12.423505345088643_f64) * t179 * t573;
    let t888 = F::cast_from(1.4760499452555382_f64) * t192 * t573;
    let t889 = t127 * t129;
    let t890 = t126 * t889;
    (t882, t884, t886, t888, t889, t890)
}
