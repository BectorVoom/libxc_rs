//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 521/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk521<F: Float>(t1990: F, t2042: F, t2126: F, t2196: F, t1573: F, t1574: F, t1577: F, t1581: F, t1584: F, t1588: F, t1592: F, t1595: F, t1599: F, t1603: F, t1606: F, t163: F, t164: F, t169: F, t171: F, t1902: F, t1905: F, t1908: F, t1912: F, t1919: F) -> (F, F) {
    let t2198 = t1990 + t2042 + t2126 + t2196;
    let t2203 = -t1573 + 0.031505407223141116 * t1574 + t1577 + t1581 + 0.031505407223141116 * t1902 - 0.031505407223141116 * t1905 * t164 - 0.031505407223141116 * t1908 - 0.001975389032890948 * t1912 - 0.031505407223141116 * t1584 - t1592 - t1595 - 0.001975389032890948 * t1588 - t1599 - t1603 + 0.008980675507690957 * t1606 + 0.008980675507690957 * t1919 - 0.005388405304614574 * t169 * t171 * t2198 * t163;
    (t2198, t2203)
}
