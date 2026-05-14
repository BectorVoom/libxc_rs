//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1055/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1055<F: Float>(t3675: F, t7520: F, t1325: F, t1440: F, t494: F, t519: F, t542: F, t9223: F, t2098: F, t6903: F, t34: F, t4956: F, t6997: F, t3787: F, t7604: F, t7600: F) -> (F, F, F, F, F, F) {
    let t21945 = t3675 * t7520;
    let t21949 = 8.0 / 5.0 * t1325 * t1440 * t21945 * t494;
    let t21954 = 16.0 / 5.0 * t519 * t1440 * t9223 * t7520 * t542;
    let t21958 = 12.0 / 5.0 * t519 * t1440 * t6903 * t2098;
    let t21962 = 4.0 / 5.0 * t1325 * t4956 * t6997 * t34;
    let t21964 = t519 * t3787 * t7604;
    let t21965 = 8.0 / 15.0 * t21964;
    let t21967 = t1325 * t3787 * t7600;
    (t21949, t21954, t21958, t21962, t21965, t21967)
}
