//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 948/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk948<F: Float>(t11666: F, t1318: F, t2192: F, t9432: F, t2089: F, t933: F, t1973: F, t925: F, t1968: F, t2092: F, t2061: F, t803: F) -> (F, F, F, F, F, F, F, F) {
    let t11667 = F::cast_from(0.07184540406152766_f64) * t11666;
    let t11677 = t1318 * t9432 * t2192;
    let t11678 = F::new(8.0) / F::new(45.0) * t11677;
    let t11695 = t933 * t2089;
    let t11709 = t925 * t1973;
    let t11753 = t925 * t1968;
    let t11754 = F::cast_from(0.03199259259259259_f64) * t11753;
    let t11781 = t933 * t2092;
    let t11829 = t2061 * t803;
    (t11667, t11678, t11695, t11709, t11753, t11754, t11781, t11829)
}
