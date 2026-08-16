//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 888/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk888(t16892: f64, t21110: f64, t1889: f64, t5732: f64, t3984: f64, t1380: f64, t6937: f64, t12194: f64, t11425: f64, t6281: f64, t833: f64) -> (f64, f64, f64, f64) {
    let t21111 = t16892 * t21110;
    let t21116 = t1889 * t5732;
    let t21117 = t3984 * t21116;
    let t21120 = t6937 * t1380;
    let t21121 = t12194 * t21120;
    let t21124 = t11425 * t6281;
    let t21125 = t21124 * t833;
    (t21111, t21117, t21121, t21125)
}
