//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 454/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk454<F: Float>(t120: F, t1832: F, t102: F, t156: F, t763: F, t426: F, t411: F, t767: F, t128: F) -> (F, F, F, F, F, F) {
    let t1833 = t120 * t1832;
    let t1835 = 2.923025 * t102 * t1833;
    let t1837 = t156 * t763;
    let t1838 = t426 * t1837;
    let t1840 = t767 * t411;
    let t1844 = t128 * t1832;
    (t1833, t1835, t1837, t1838, t1840, t1844)
}
