//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 425/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk425<F: Float>(t2159: F, t2163: F, t2167: F, t2171: F, t2175: F, t2179: F, t793: F, t794: F, t798: F, t799: F, t804: F, t89: F, t2354: F, t143: F, t151: F, t155: F, t179: F, t2210: F, t2214: F, t886: F, t888: F, t921: F, t925: F, t946: F, t959: F, t98: F, t982: F, t986: F) -> (F, F, F, F, F) {
    let t2417 = t793 + t794 + 9.625452574583042 * t2159 + 9.625452574583042 * t2163 - 9.625452574583042 * t2167 + t798 + t799 + 0.64 * t2171 + 0.64 * t2175 - 0.64 * t2179;
    let t2418 = t2417 * t804;
    let t2419 = t2418 * t89;
    let t2426 = t2354 * t89;
    let t2437 = -t886 + t888 + t921 + t925 - 1.8805371096875316 * t151 * t2214 + 1.8805371096875316 * t2419 * t98 - 19.489173774580152 * t155 * t2210 - 19.489173774580152 * t155 * t2214 + 19.489173774580152 * t2426 * t98 + 3.7610742193750633 * t143 * t2210 + 3.7610742193750633 * t143 * t2214 - 18.635258017632964 * t179 * t2210 - 18.635258017632964 * t179 * t2214 - t946 + t959 - t982 + t986;
    (t2417, t2418, t2419, t2426, t2437)
}
