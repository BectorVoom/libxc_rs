//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1264/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1264<F: Float>(t684: F, t7071: F, t1553: F, t1878: F, t1729: F, t2644: F, t2610: F, t2765: F, t440: F, t405: F, t1832: F, t7213: F, t169: F, t301: F, t6080: F, t717: F) -> (F, F, F, F, F, F, F, F) {
    let t18795 = t684 * t7071;
    let t18797 = t1553 * t1878;
    let t18801 = t1729 * t2644;
    let t18805 = t2765 * t2610 * t440;
    let t18809 = t405 * t2644 * t1553;
    let t18826 = t1832 * t1832;
    let t18830 = t405 * t7213;
    let t18835 = t169 * t717 * t6080 * t301;
    (t18795, t18797, t18801, t18805, t18809, t18826, t18830, t18835)
}
