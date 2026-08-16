//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 627/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk627(t5491: f64, t5492: f64, t1775: f64, t1849: f64, t786: f64, t3290: f64, t2014: f64, t3293: f64, t2019: f64, t785: f64, t657: f64, t2023: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5493 = t5491 * t5492;
    let t5494 = t1775 * t5493;
    let t5497 = t786 * t1849;
    let t5498 = t5497 * t3290;
    let t5499 = t1775 * t5498;
    let t5502 = t2014 * t3293;
    let t5503 = t1775 * t5502;
    let t5507 = 1.0_f64 / t2019 / t785;
    let t5508 = t657 * t5507;
    let t5509 = t2023 * t2023;
    (t5493, t5494, t5497, t5498, t5499, t5502, t5503, t5507, t5508, t5509)
}
