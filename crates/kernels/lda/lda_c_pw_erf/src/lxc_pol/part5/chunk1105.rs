//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1105/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1105<F: Float>(t11597: F, t14469: F, t14470: F, t1556: F, t1734: F, t1832: F, t1859: F, t18797: F, t1881: F, t2211: F, t2630: F, t2645: F, t2675: F, t2765: F, t411: F, t4441: F, t5735: F, t5783: F, t6086: F, t6087: F, t6089: F, t6129: F, t7075: F, t756: F, t770: F, t777: F, t7886: F, t7887: F, t7977: F, t8004: F, t9163: F) -> F {
    let t20586 = F::cast_from(9.0_f64) * t5735 * t8004 - F::cast_from(2.0_f64) * t2645 * t6087 - F::cast_from(3.0_f64) * t5783 * t2765 * t2630 * t411 - F::cast_from(2.7743564462147594_f64) * t11597 + t1881 * t7887 - F::cast_from(9.0_f64) * t5783 * t2765 * t770 * t1832 - F::cast_from(2.0_f64) * t777 * t18797 * t2675 - F::cast_from(2.0_f64) * t777 * t6086 * t6129 - F::cast_from(9.0_f64) * t5783 * t2765 * t1859 * t756 - F::cast_from(2.0_f64) * t2645 * t7075 - t9163 + F::cast_from(9.0_f64) * t6089 * t4441 - t7977 * t1556 + F::cast_from(3.0_f64) * t2211 * t7886 * t1734 + t14469 + F::cast_from(0.17961351015381913_f64) * t14470;
    t20586
}
