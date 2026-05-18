//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1086/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1086<F: Float>(t300: F, t6404: F, t2255: F, t2277: F, t356: F, t18439: F, t18442: F, t6141: F, t828: F, t6312: F, t858: F, t6121: F, t877: F) -> (F, F, F, F, F, F, F, F, F) {
    let t18661 = t300 * t6404;
    let t18706 = t356 / t2277 / t2255;
    let t18750 = F::new(0.17757530864197530864e0) * t18439;
    let t18765 = F::new(0.5356037037037037037e1) * t18439;
    let t18766 = F::new(0.16979925925925925926e1) * t18442;
    let t18790 = t828 * t6141;
    let t18843 = F::new(0.18467901234567901234e0) * t18439;
    let t18854 = t858 * t6312;
    let t18863 = t877 * t6121;
    (t18661, t18706, t18750, t18765, t18766, t18790, t18843, t18854, t18863)
}
