//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 978/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk978(t3970: f64, t3989: f64, t4056: f64, t550: f64, t543: f64, t3992: f64, t2661: f64, t240: f64, t4000: f64, t4003: f64, t9768: f64, t532: f64, t549: f64) -> (f64, f64, f64, f64, f64) {
    let t9926 = t3989 * t3970;
    let t9929 = t550 * t4056;
    let t9930 = t9929 * t543;
    let t9931 = t3992 * t9930;
    let t9932 = t2661 * t9931;
    let t9934 = t4000 * t240;
    let t9935 = t9768 * t4003;
    let t9936 = t9934 * t9935;
    let t9937 = t2661 * t9936;
    let t9940 = 1.0_f64 / t549 / t532;
    (t9926, t9932, t9934, t9937, t9940)
}
