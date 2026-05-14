//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 128/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk128<F: Float>(t280: F, t395: F, t287: F, t294: F, t305: F) -> (F, F, F, F) {
    let t396 = t395 * t280;
    let t401 = 3.125 * t287 + 1.2466946262544771 * t294 + 0.146484375;
    let t402 = f64::ln(t401);
    let t403 = t402 * t305;
    (t396, t401, t402, t403)
}
