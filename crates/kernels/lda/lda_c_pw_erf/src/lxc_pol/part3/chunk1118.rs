//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1118/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1118<F: Float>(t13985: F, t13986: F, t13987: F, t13988: F, t13989: F, t13990: F, t13991: F, t13992: F, t13993: F, t13994: F, t13995: F, t13998: F, t13999: F, t14000: F, t14001: F, t14002: F, t14005: F, t14007: F, t14010: F, t14013: F, t14017: F, t14020: F, t14022: F, t14025: F, t14029: F, t14033: F, t14037: F) -> (F, F) {
    let t15116 = t13985 + t13986 + t13987 + t13988 + t13989 + t13990 + t13991 - t13992 - t13993 - t13994 - t13995 - t13998 + t13999;
    let t15117 = t14000 + t14001 + t14002 + t14005 - t14007 - t14010 - t14013 - t14017 - t14020 - t14022 - t14025 - t14029 - t14033 + t14037;
    (t15116, t15117)
}
