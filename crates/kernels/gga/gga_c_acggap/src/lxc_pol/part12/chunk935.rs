//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 935/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk935<F: Float>(t17912: F, t2302: F, t31443: F, t3176: F, t1530: F, t31056: F, t13287: F, t33953: F, t5136: F, t5141: F, t15386: F, t3073: F, t4241: F, t13364: F, t13299: F, t2001: F, t4344: F) -> (F, F, F, F, F, F, F, F) {
    let t34821 = t31443 * t17912 * t2302 * t3176;
    let t34823 = t1530 * t31056;
    let t34826 = t34823 * t13287 * t33953 * t5136;
    let t34828 = t33953 * t5141;
    let t34830 = t34823 * t15386 * t34828;
    let t34833 = t3073 * t31056;
    let t34834 = t33953 * t4241;
    let t34836 = t34833 * t13364 * t34834;
    let t34839 = t34833 * t13299 * t34834;
    let t34841 = t2001 * t4344;
    (t34821, t34823, t34826, t34828, t34830, t34836, t34839, t34841)
}
