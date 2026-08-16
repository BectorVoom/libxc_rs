//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 518/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk518(t1477: f64, t5481: f64, t542: f64, t1098: f64, t1996: f64, t1961: f64, t531: f64, t833: f64, t3766: f64, t518: f64, t1319: f64, t3786: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5482 = t1477 * t5481;
    let t5483 = t542 * t5482;
    let t5486 = t1098 * t1996;
    let t5488 = t1961 * t531;
    let t5489 = t5488 * t833;
    let t5490 = t3766 * t5489;
    let t5493 = t518 * t1961;
    let t5494 = t5493 * t1319;
    let t5495 = t3786 * t5494;
    (t5482, t5483, t5486, t5489, t5490, t5494, t5495)
}
