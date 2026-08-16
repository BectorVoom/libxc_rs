//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta865 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2758;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2759;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta865(t13845: f64, t13847: f64, t5675: f64, t73731: f64, t3938: f64, t9816: f64, t9818: f64, t13848: f64, t5659: f64, t22159: f64, t48836: f64, t22120: f64, t9962: f64, t46917: f64, t6871: f64, t22298: f64, t48862: f64, t48863: f64, t22098: f64, t22102: f64, t46740: f64, t22299: f64, t22295: f64, t22111: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t73734, t73738, t73742, t73744, t73750) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2758(t13845, t13847, t5675, t73731, t3938, t9816, t9818, t13848, t5659, t22159, t48836, t22120, t9962);
        let (t73778, t73781, t73787, t73789, t73798, t73800, t73803) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2759(t46917, t6871, t22298, t48862, t48863, t22098, t9962, t22102, t46740, t22299, t22295, t22111);
    (t73734, t73738, t73742, t73744, t73750, t73778, t73781, t73787, t73789, t73798, t73800, t73803)
}
