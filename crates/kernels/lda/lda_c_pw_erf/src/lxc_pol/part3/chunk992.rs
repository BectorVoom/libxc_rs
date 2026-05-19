//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 992/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk992<F: Float>(t11567: F, t1832: F, t2765: F, t440: F, t1553: F, t1880: F, t405: F, t10832: F, t4429: F, t1809: F, t2790: F, t169: F, t2817: F, t301: F, t865: F) -> (F, F, F, F, F, F) {
    let t11568 = F::cast_from(0.5945049527603057_f64) * t11567;
    let t11570 = t2765 * t1832 * t440;
    let t11574 = t405 * t1880 * t1553;
    let t11577 = t10832 * t4429;
    let t11588 = t2790 * t1809;
    let t11597 = t169 * t2817 * t865 * t301;
    (t11568, t11570, t11574, t11577, t11588, t11597)
}
