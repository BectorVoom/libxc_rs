//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta297 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1545;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1546;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta297(t760: f64, t9323: f64, t9318: f64, t2251: f64, t750: f64, t2611: f64, t10467: f64, t162: f64, t187: f64, t2398: f64, t2615: f64, t2609: f64, t717: f64, t9544: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10552, t10554, t10555, t10556, t10557, t10558, t10560, t10561, t10562, t10563) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1545(t760, t9323, t9318, t2251, t750, t2611, t10467, t162, t187, t2398, t2615, t2609, t717);
        let (t10564, t10565) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1546(t10563, t162, t9544);
    (t10552, t10554, t10555, t10556, t10557, t10558, t10560, t10561, t10562, t10563, t10564, t10565)
}
