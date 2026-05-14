//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1266/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1266<F: Float>(t1184: F, t1187: F, t2363: F, t483: F, t10832: F, t14485: F, t14488: F, t1664: F, t1704: F, t1859: F, t2765: F, t2798: F, t411: F, t440: F, t6086: F, t6154: F, t6155: F, t770: F, t777: F, t8759: F, t8771: F, t8774: F, t8789: F, t8793: F, t8805: F, t8808: F, t8812: F, t8816: F, t8821: F, t8822: F, t8825: F) -> (F,) {
    let t18866 = t1184 * t2363 * t483 * t1187;
    let t18869 = t8759 + 4.0 * t6154 * t10832 * t6155 - 6.0 * t14488 * t2765 * t770 * t1664 + 4.0 * t6154 * t2765 * t1859 * t440 + 2.0 * t6154 * t2765 * t770 * t1704 + 12.0 * t14485 * t2765 * t6155 * t411 + 0.11974234010254609 * t8771 + t8774 - 0.01197423401025461 * t8789 - 0.02394846802050922 * t8793 - t8805 - 9.138438188948293e-06 * t8808 - t8812 + 0.039914113367515366 * t8816 + t8821 - t777 * t6086 * t2798 - 1.82185769317151e-05 * t18866 + 0.3902713307045947 * t8822 + t8825;
    (t18869,)
}
