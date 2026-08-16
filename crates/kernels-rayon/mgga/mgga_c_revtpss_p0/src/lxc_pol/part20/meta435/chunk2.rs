//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1641/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1641(t1214: f64, t12621: f64, t12956: f64, t12959: f64, t3566: f64, t3781: f64, t5330: f64, t3362: f64, t404: f64, t43766: f64, t1222: f64, t13007: f64, t140: f64) -> (f64, f64, f64, f64, f64) {
    let t44944 = t12621 * t1214;
    let t44949 = t12956 * t12959;
    let t44951 = t3566 * t3781;
    let t44952 = t44951 * t5330;
    let t44958 = 1.0_f64 / t404 / t3362;
    let t44959 = t44958 * t43766;
    let t44965 = t1222 * t140 * t13007;
    (t44944, t44949, t44952, t44959, t44965)
}
