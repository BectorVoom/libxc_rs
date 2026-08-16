//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1188/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1188<F: Float>(t10607: F, t13983: F, t13984: F, t13985: F, t13986: F, t13987: F, t13988: F, t13989: F, t13990: F, t13991: F, t13992: F, t13993: F, t13994: F) -> (F, F) {
    let t13995 = F::cast_from(8.0_f64) / F::cast_from(81.0_f64) * t10607;
    let t13996 = t13983 + t13984 + t13985 + t13986 + t13987 + t13988 + t13989 + t13990 + t13991 - t13992 - t13993 - t13994 - t13995;
    (t13995, t13996)
}
