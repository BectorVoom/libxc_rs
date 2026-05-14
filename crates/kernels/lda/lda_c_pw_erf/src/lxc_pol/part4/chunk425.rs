//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 425/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk425<F: Float>(t1624: F, t1625: F, t1642: F, t1643: F, t1203: F, t1573: F, t1574: F, t1577: F, t1581: F, t1584: F, t1588: F, t1592: F, t1595: F, t1599: F, t1603: F, t1606: F, t163: F, t164: F, t169: F, t171: F) -> (F, F) {
    let t1645 = t1624 + t1625 + t1642 + t1643;
    let t1650 = -t1573 + 0.06301081444628223 * t1574 + t1577 + t1581 - 0.031505407223141116 * t1203 * t164 - 0.06301081444628223 * t1584 - 0.003950778065781896 * t1588 - t1592 - t1595 - t1599 - t1603 + 0.017961351015381915 * t1606 - 0.005388405304614574 * t169 * t171 * t1645 * t163;
    (t1645, t1650)
}
