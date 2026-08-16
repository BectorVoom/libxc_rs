//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 919/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk919(t19715: f64, t9410: f64, t14627: f64, t1126: f64, t6482: f64, t303: f64, t1662: f64, t4813: f64, t14067: f64, t3200: f64, t19710: f64, t4580: f64) -> (f64, f64, f64, f64, f64) {
    let t19716 = t9410 * t19715;
    let t19717 = t14627 * t19716;
    let t19719 = t6482 * t1126;
    let t19720 = t303 * t19719;
    let t19723 = t1662 * t4813;
    let t19724 = t14067 * t19723;
    let t19725 = t3200 * t19724;
    let t19727 = t4580 * t19710;
    (t19717, t19720, t19723, t19725, t19727)
}
