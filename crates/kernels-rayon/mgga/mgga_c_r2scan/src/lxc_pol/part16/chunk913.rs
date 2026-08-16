//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 913/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk913(t2266: f64, t2267: f64, t3016: f64, t2526: f64, t2854: f64, t4873: f64, t5039: f64, t7156: f64, t8653: f64, t8654: f64, t8655: f64, t8656: f64, t8657: f64, t8658: f64, t881: f64, t9069: f64, t9072: f64) -> f64 {
    let t9824 = t2266 * t2267 * t3016;
    let t9825 = 3.0_f64 * t9824;
    let t9827 = t2266 * t2854 * t2526;
    let t9828 = 6.0_f64 * t9827;
    let t9829 = -0.4726e1_f64 * t881 * t9069 - 0.4726e1_f64 * t881 * t9072 + t8653 + t8654 + t8655 - t4873 - t9825 + t7156 + t8656 + t8657 - t8658 - t9828 - t5039;
    t9829
}
