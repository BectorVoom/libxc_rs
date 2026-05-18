//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 810/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk810<F: Float>(t436: F, t5548: F, t120: F, t102: F, t3296: F, t756: F, t1664: F, t767: F, t1697: F, t1832: F, t1844: F, t411: F) -> (F, F, F, F, F, F, F) {
    let t5565 = t436 * t5548;
    let t5568 = t120 * t5548;
    let t5570 = F::new(2.923025) * t102 * t5568;
    let t5571 = t3296 * t756;
    let t5577 = F::new(17.53815) * t102 * t767 * t1664;
    let t5578 = t1697 * t1832;
    let t5588 = F::new(11.6921) * t102 * t1844 * t411;
    (t5565, t5568, t5570, t5571, t5577, t5578, t5588)
}
