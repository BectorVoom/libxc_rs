//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1435/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1435(t13784: f64, t13790: f64, t13789: f64, t13880: f64, t13943: f64, t13949: f64, t13954: f64, t13956: f64, t5671: f64, t9776: f64, t9780: f64, t9786: f64, t9791: f64, t9796: f64, t9799: f64) -> f64 {
    let t22145 = t13790 * t13784;
    let t22146 = t13789 * t22145;
    let t22153 = -0.76220476654346199061e-4_f64 * t9776 - 0.22675591804667994221e-1_f64 * t9780 + t13880 - 0.34299214494455789578e-2_f64 * t5671 * t22146 - t9786 - t9791 - 0.45178982497454656791e-5_f64 * t9796 - 0.18071592998981862716e-4_f64 * t9799 + t13943 - 0.60976381323476959249e-3_f64 * t13949 + t13954 + 0.50820002809285328224e-5_f64 * t13956;
    t22153
}
