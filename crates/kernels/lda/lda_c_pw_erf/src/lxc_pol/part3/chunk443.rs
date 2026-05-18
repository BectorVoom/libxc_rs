//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 443/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk443<F: Float>(t1664: F, t1697: F, t127: F, t1568: F, t1655: F, t1661: F, t1663: F, t1667: F, t1670: F, t1674: F, t1676: F, t1679: F, t1683: F, t1689: F, t1692: F, t1695: F, t426: F, t436: F) -> F {
    let t1698 = t1697 * t1664;
    let t1704 = -t1655 + t1661 + t1663 + t1667 - t1670 + t1674 + t1676 / F::new(3.0) + F::new(3.0) / F::new(2.0) * t426 * t1679 - t426 * t1683 / F::new(2.0) + t1689 + F::new(1.46904) * t1692 + t1695 + F::new(5.87616) * t127 * t1698 - F::new(1.46904) * t127 * t436 * t1568;
    t1704
}
