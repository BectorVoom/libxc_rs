//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1369/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1369<F: Float>(t11266: F, t15296: F, t18991: F, t18992: F, t18993: F, t18994: F, t18995: F, t18996: F, t18997: F, t18999: F, t19001: F, t8527: F, t8533: F, t8536: F, t8539: F, t8542: F, t8716: F, t8733: F, t8737: F, t8740: F) -> (F,) {
    let t19892 = -t18991 + t8527 + t18992 + t8533 - t8536 + t8539 - t8542 - t11266 + t18993 - t18994 - t18995 + 2.7384219226711113 * t15296 + t18996 - t18997 + t18999 + t8733 + t19001 - t8716 - t8737 + t8740;
    (t19892,)
}
