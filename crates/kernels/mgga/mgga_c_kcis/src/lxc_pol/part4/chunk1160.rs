//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1160/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1160<F: Float>(t12217: F, t498: F, t16055: F, t3977: F, t736: F, t16065: F, t1889: F, t4007: F, t3984: F, t1444: F, t1938: F, t2642: F, t12147: F, t5722: F, t1368: F, t531: F, t5732: F) -> (F, F, F, F, F, F, F) {
    let t16901 = t12217 * t498;
    let t16902 = t16901 * t16055;
    let t16905 = t736 * t3977;
    let t16906 = t16905 * t498;
    let t16907 = t16906 * t16065;
    let t16910 = t1889 * t4007;
    let t16911 = t3984 * t16910;
    let t16919 = t1938 * t1444 * t2642;
    let t16920 = t3984 * t16919;
    let t16923 = t12147 * t5722;
    let t16925 = t1368 * t16923 / 432.0;
    let t16926 = t5732 * t531;
    (t16902, t16905, t16907, t16911, t16920, t16925, t16926)
}
