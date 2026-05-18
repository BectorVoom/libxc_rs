//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 532/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk532<F: Float>(t2657: F, t2663: F, t2665: F, t2666: F, t1573: F, t1577: F, t1581: F, t1592: F, t1595: F, t1599: F, t1603: F, t163: F, t164: F, t169: F, t171: F, t1902: F, t1908: F, t1912: F, t1919: F, t2379: F) -> (F, F) {
    let t2668 = t2657 + t2663 + t2665 + t2666;
    let t2673 = -t1573 + F::new(0.06301081444628223) * t1902 + t1577 + t1581 - F::new(0.031505407223141116) * t2379 * t164 - F::new(0.06301081444628223) * t1908 - F::new(0.003950778065781896) * t1912 - t1592 - t1595 - t1599 - t1603 + F::new(0.017961351015381915) * t1919 - F::new(0.005388405304614574) * t169 * t171 * t2668 * t163;
    (t2668, t2673)
}
