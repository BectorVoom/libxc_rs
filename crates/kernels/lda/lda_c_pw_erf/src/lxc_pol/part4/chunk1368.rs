//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1368/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1368<F: Float>(t11250: F, t11254: F, t11256: F, t11260: F, t18983: F, t18984: F, t18985: F, t18986: F, t18987: F, t18988: F, t18989: F, t8469: F, t8473: F, t8477: F, t8481: F, t8491: F, t8493: F, t8505: F, t8509: F, t8516: F) -> (F,) {
    let t19890 = -t18983 + t8469 + t8473 - t8477 + t11250 - t8481 + t18984 - t18985 + t8491 + t8493 - t18986 + t18987 - 0.9480012043054112 * t11254 - t8505 + t8509 + t18988 + 6.327242966164847 * t11256 - 0.6327242966164848 * t11260 + t18989 + t8516;
    (t19890,)
}
