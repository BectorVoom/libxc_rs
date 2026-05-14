//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1087/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1087<F: Float>(t11871: F, t15823: F, t15825: F, t10145: F, t15788: F, t15792: F, t15796: F, t15798: F, t15800: F, t15804: F, t15809: F, t15813: F, t15818: F, t15820: F, t325: F, t6501: F) -> (F, F, F) {
    let t15827 = t15823 * t11871 * t15825;
    let t15829 = -0.0012594444444444445 * t15788 + 0.010075555555555556 * t15792 + 0.011335 * t15796 - 0.0013993827160493828 * t15798 - 0.01847185185185185 * t15800 - 0.007556666666666666 * t15804 - 0.02518888888888889 * t15809 + 0.002099074074074074 * t15813 + 0.005597530864197531 * t15818 - t10145 - 0.007556666666666666 * t15820 + 0.01679259259259259 * t15827;
    let t15836 = t325 * t6501;
    (t15827, t15829, t15836)
}
