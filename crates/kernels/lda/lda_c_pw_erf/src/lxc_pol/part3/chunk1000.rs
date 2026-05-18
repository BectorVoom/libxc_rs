//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1000/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1000<F: Float>(t11677: F, t1318: F, t3899: F, t5366: F, t1466: F, t2191: F, t3655: F, t9217: F, t1966: F, t2961: F, t4619: F, t945: F) -> (F, F, F, F, F, F) {
    let t11678 = F::new(8.0) / F::new(45.0) * t11677;
    let t11680 = t1318 * t3899 * t5366;
    let t11681 = F::new(8.0) / F::new(15.0) * t11680;
    let t11685 = F::new(4.0) / F::new(15.0) * t1318 * t1466 * t2191 * t3655;
    let t11686 = F::new(4.0) / F::new(45.0) * t9217;
    let t11687 = t1966 * t2961;
    let t11691 = t4619 * t945;
    (t11678, t11681, t11685, t11686, t11687, t11691)
}
