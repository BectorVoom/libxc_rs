//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 448/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk448<F: Float>(t120: F, t133: F, t474: F, t1675: F, t1655: F, t1661: F, t1663: F, t1667: F, t1670: F, t1679: F, t1683: F) -> (F, F, F) {
    let t1717 = F::new(0.3831677777777778) * t133 * t474 * t120;
    let t1718 = t133 * t1675;
    let t1724 = -t1655 + t1661 + t1663 + t1667 - t1670 + t1717 + F::new(1.1495033333333333) * t1718 + F::new(5.172765) * t133 * t1679 - F::new(1.724255) * t133 * t1683;
    (t1717, t1718, t1724)
}
