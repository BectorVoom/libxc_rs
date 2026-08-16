//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1317/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1317(t12217: f64, t498: f64, t16055: f64, t3977: f64, t736: f64, t16065: f64, t1889: f64, t4007: f64, t3984: f64, t1444: f64, t1938: f64, t2642: f64) -> (f64, f64, f64, f64, f64) {
    let t16901 = t12217 * t498;
    let t16902 = t16901 * t16055;
    let t16905 = t736 * t3977;
    let t16906 = t16905 * t498;
    let t16907 = t16906 * t16065;
    let t16910 = t1889 * t4007;
    let t16911 = t3984 * t16910;
    let t16919 = t1938 * t1444 * t2642;
    (t16902, t16905, t16907, t16911, t16919)
}
