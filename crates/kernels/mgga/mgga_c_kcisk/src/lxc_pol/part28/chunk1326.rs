//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1326/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1326<F: Float>(t32889: F, t9932: F, t3805: F, t9960: F, t1772: F, t648: F, t64908: F, t7233: F, t9650: F, t17357: F, t33031: F, t34017: F, t33003: F, t5014: F, t9670: F, t18325: F, t34072: F) -> (F, F, F, F, F, F, F, F, F) {
    let t116886 = t9932 * t32889;
    let t116888 = t3805 * t9960;
    let t116903 = t64908 * t648 * t1772;
    let t116914 = t7233 * t9650;
    let t116921 = 0.30864197530864197531e-2 * t33031 * t17357 * t34017;
    let t116922 = t5014 * t33003;
    let t116932 = t5014 * t9670;
    let t116942 = t7233 * t9670;
    let t116960 = t34072 * t18325;
    (t116886, t116888, t116903, t116914, t116921, t116922, t116932, t116942, t116960)
}
