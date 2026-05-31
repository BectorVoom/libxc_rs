//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 233/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk233<F: Float>(t265: F, t668: F, t256: F, t267: F, t498: F, t513: F, t517: F, t527: F, t546: F, t553: F, t567: F, t570: F, t579: F, t597: F, t640: F, t649: F, t655: F, t658: F, t665: F) -> (F, F) {
    let t670 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t265 * t668;
    let t671 = t498 + t513 + t517 + t527 - t546 + t553 + t567 + t570 + t579 - t597 + t640 * t256 / F::cast_from(3.0_f64) + t649 + t655 + t658 - t665 * t267 / F::cast_from(15.0_f64) - t670;
    (t670, t671)
}
