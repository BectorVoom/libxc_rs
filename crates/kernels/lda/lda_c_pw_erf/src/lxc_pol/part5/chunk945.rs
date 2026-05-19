//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 945/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk945<F: Float>(t1187: F, t2824: F, t483: F, t780: F, t1738: F, t2310: F, t1191: F, t169: F, t1891: F, t301: F, t1553: F, t1880: F, t405: F) -> (F, F, F, F) {
    let t11561 = t2824 * t780 * t483 * t1187;
    let t11563 = t1738 * t2310;
    let t11567 = t169 * t1191 * t1891 * t301;
    let t11568 = F::cast_from(0.5945049527603057_f64) * t11567;
    let t11574 = t405 * t1880 * t1553;
    (t11561, t11563, t11568, t11574)
}
