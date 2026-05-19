//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1329/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1329<F: Float>(t11250: F, t11254: F, t11256: F, t11260: F, t14429: F, t14430: F, t14431: F, t14432: F, t14433: F, t14434: F, t14436: F, t14437: F, t8473: F, t8477: F, t8481: F, t8491: F, t8505: F, t8509: F, t8516: F) -> F {
    let t15295 = t8473 - t8477 + t11250 - t8481 + t14429 - t14430 + t8491 + t14431 - t14432 - t14433 - t14434 - F::cast_from(1.4220018064581168_f64) * t11254 - t8505 + t8509 + t14436 + F::cast_from(9.49086444924727_f64) * t11256 - F::cast_from(1.898172889849454_f64) * t11260 - t14437 + t8516;
    t15295
}
