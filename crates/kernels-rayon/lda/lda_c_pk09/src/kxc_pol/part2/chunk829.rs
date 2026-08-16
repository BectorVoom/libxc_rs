//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 829/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk829(t2269: f64, t733: f64, t204: f64, t737: f64, t2152: f64, t823: f64, t825: f64, t96: f64, t8092: f64, t831: f64, t957: f64, t2318: f64, t623: f64, t844: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8392 = t2269 * t733;
    let t8393 = t8392 * t204;
    let t8394 = t8393 * t737;
    let t8404 = t96 * t2152 * t823 * t825;
    let t8407 = t831 * t8092;
    let t8413 = t957 * t8092;
    let t8416 = t844 * t2318 * t623;
    (t8392, t8394, t8404, t8407, t8413, t8416)
}
