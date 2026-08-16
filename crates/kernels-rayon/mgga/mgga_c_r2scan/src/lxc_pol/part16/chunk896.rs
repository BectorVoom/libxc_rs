//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 896/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk896(t1048: f64, t795: f64, t9573: f64, t910: f64, t2266: f64, t2867: f64, t2267: f64, t2892: f64, t2858: f64, t2526: f64, t2859: f64, t2333: f64, t3245: f64) -> (f64, f64, f64, f64, f64) {
    let t9575 = t1048 * t9573 * t795;
    let t9576 = 2.0_f64 * t9575;
    let t9577 = t910 * t795;
    let t9579 = t2266 * t2867 * t9577;
    let t9580 = 6.0_f64 * t9579;
    let t9583 = t2267 * t2892;
    let t9584 = t2858 * t9583;
    let t9585 = 6.0_f64 * t9584;
    let t9586 = t2859 * t2526;
    let t9587 = t2858 * t9586;
    let t9588 = 12.0_f64 * t9587;
    let t9589 = t3245 * t2333;
    (t9576, t9580, t9585, t9588, t9589)
}
