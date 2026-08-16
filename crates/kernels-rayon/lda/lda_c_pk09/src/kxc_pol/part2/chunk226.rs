//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 226/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk226(t200: f64, t573: f64, t205: f64, t569: f64, t179: f64, t192: f64, t127: f64, t129: f64, t126: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t882 = 1.6183441301295518_f64 * t200 * t573;
    let t884 = 1.5795946107075225_f64 * t205 * t569;
    let t886 = 12.423505345088643_f64 * t179 * t573;
    let t888 = 1.4760499452555382_f64 * t192 * t573;
    let t889 = t127 * t129;
    let t890 = t126 * t889;
    (t882, t884, t886, t888, t889, t890)
}
