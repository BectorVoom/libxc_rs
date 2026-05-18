//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1023/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1023<F: Float>(t11989: F, t1318: F, t5271: F, t11960: F, t11961: F, t11962: F, t11963: F, t11964: F, t11965: F, t11966: F, t11968: F, t11973: F, t11978: F, t11982: F, t11988: F) -> (F, F) {
    let t11991 = t1318 * t11989 * t5271;
    let t11992 = F::new(32.0) / F::new(15.0) * t11991;
    let t11993 = t11960 - t11961 + t11962 + t11963 - t11964 + t11965 + t11966 + t11968 + t11973 + t11978 + t11982 - t11988 + t11992;
    (t11992, t11993)
}
