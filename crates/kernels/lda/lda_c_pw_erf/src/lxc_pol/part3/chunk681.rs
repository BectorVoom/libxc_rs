//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 681/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk681<F: Float>(t352: F, t4675: F, t1349: F, t1953: F, t1943: F, t954: F, t11: F, t951: F, t557: F, t1333: F, t34: F, t1948: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t4676 = t4675 * t352;
    let t4677 = t1349 * t4676;
    let t4678 = t1953 * t4677;
    let t4680 = t1943 * t954;
    let t4681 = t1349 * t4680;
    let t4682 = t11 * t4681;
    let t4684 = t1943 * t951;
    let t4685 = t557 * t4684;
    let t4686 = t11 * t4685;
    let t4688 = t1333 * t34;
    let t4689 = t4688 * t352;
    let t4690 = t557 * t4689;
    let t4691 = t1953 * t4690;
    let t4693 = t1948 * t954;
    let t4694 = t557 * t4693;
    let t4695 = t11 * t4694;
    (t4676, t4677, t4678, t4680, t4681, t4682, t4684, t4685, t4686, t4688, t4689, t4690, t4691, t4693, t4694, t4695)
}
