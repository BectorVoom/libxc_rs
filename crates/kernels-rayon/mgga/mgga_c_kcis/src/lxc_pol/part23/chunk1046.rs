//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1046/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1046(t4173: f64, t7923: f64, t1394: f64, t2645: f64, t7909: f64, t5709: f64, t1386: f64, t3754: f64) -> (f64, f64, f64, f64, f64) {
    let t27431 = t7923 * t4173;
    let t27432 = t1394 * t27431;
    let t27434 = t7909 * t2645;
    let t27435 = t5709 * t27434;
    let t27438 = t1386 * t3754;
    (t27431, t27432, t27434, t27435, t27438)
}
