//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 500/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk500(t2824: f64, t467: f64, t452: f64, t2758: f64, t471: f64, t1782: f64, t2778: f64) -> (f64, f64, f64, f64) {
    let t2825 = t467 * t2824;
    let t2826 = t2825 * t452;
    let t2829 = t471 * t2758;
    let t2832 = t2778 * t1782;
    (t2825, t2826, t2829, t2832)
}
