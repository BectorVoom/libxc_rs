//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 327/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk327(t1009: f64, t1709: f64, t1022: f64, t1662: f64, t1021: f64, t1020: f64, t1646: f64, t313: f64) -> (f64, f64, f64, f64, f64) {
    let t1710 = t1709 * t1009;
    let t1713 = t1022 * t1662;
    let t1714 = t1021 * t1713;
    let t1715 = t1020 * t1714;
    let t1717 = t313 * t1646;
    (t1710, t1713, t1714, t1715, t1717)
}
