//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 947/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk947<F: Float>(t4263: F, t781: F, t8327: F, t1590: F, t1905: F, t164: F, t4437: F, t1191: F, t163: F, t169: F, t841: F, t2198: F, t717: F) -> (F, F, F, F, F, F) {
    let t11633 = t781 * t4263;
    let t11635 = F::cast_from(96.0_f64) * t8327;
    let t11642 = t1905 * t1590;
    let t11643 = F::cast_from(0.09451622166942335_f64) * t11642;
    let t11644 = t4437 * t164;
    let t11652 = t169 * t1191 * t841 * t163;
    let t11666 = t169 * t717 * t2198 * t163;
    (t11633, t11635, t11643, t11644, t11652, t11666)
}
