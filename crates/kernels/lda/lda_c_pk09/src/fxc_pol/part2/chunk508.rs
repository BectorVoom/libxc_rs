//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 508/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk508<F: Float>(t1755: F, t1766: F, t1771: F, t1773: F, t2733: F, t2736: F, t2803: F, t2807: F, t1776: F, t452: F, t2730: F, t537: F) -> (F, F, F, F) {
    let t2938 = t1755 - F::new(6.25) * t2803 + t1766 + F::new(6.25) * t2807 + t1771 - F::new(1.2466946262544771) * t2733 + t1773 + F::new(1.2466946262544771) * t2736;
    let t2939 = t2938 * t1776;
    let t2940 = t2939 * t452;
    let t2943 = t537 * t2730;
    (t2938, t2939, t2940, t2943)
}
