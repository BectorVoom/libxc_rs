//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1298/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1298<F: Float>(t16678: F, t16680: F, t16683: F, t16687: F, t16692: F, t16696: F, t16698: F, t16700: F, t16703: F, t16708: F, t16710: F, t16712: F, t16716: F, t16721: F, t16723: F, t16727: F, t16730: F) -> (F,) {
    let t19190 = -t16678 - t16680 - t16683 + t16687 + t16692 + t16696 - t16698 + t16700 + t16703 - t16708 - t16710 + t16712 - t16716 + t16721 + t16723 + t16727 + t16730;
    (t19190,)
}
