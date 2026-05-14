//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1362/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1362<F: Float>(t14535: F, t14724: F, t1706: F, t1711: F, t1712: F, t1724: F, t1864: F, t1878: F, t19703: F, t19723: F, t19726: F, t19733: F, t19768: F, t19777: F, t19788: F, t2634: F, t2642: F, t3329: F, t3339: F, t444: F, t450: F, t5618: F, t5630: F, t7168: F, t7211: F, t774: F, t9068: F) -> (F,) {
    let t19794 = -2.0 * t19703 * t450 - 24.0 * t3339 * t1864 * t1878 - 4.0 * t5618 * t1878 - 2.0 * t1706 * t7211 - 2.0 * t14535 * t774 - 12.0 * t14724 * t5630 - 6.0 * t3339 * t2642 * t1712 + 24.0 * t9068 * t2634 * t1712 + 4.0 * t1711 * t19723 + 2.0 * t19726 * t1712 - t444 * (t19733 + t19768 + t19777 + t19788) - t7168 * t1724 - t3329 * t2642;
    (t19794,)
}
