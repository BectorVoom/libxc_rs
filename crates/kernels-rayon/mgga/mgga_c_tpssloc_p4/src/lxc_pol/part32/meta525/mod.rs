//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta525 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1859;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta525(t25049: f64, t25277: f64, t25077: f64, t25080: f64, t25140: f64, t25144: f64, t25293: f64, t25317: f64, t25211: f64, t25346: f64, t26198: f64, t26200: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t26591, t26613, t26619, t26621, t26644, t26646, t26667, t26673, t26712, t26726, t26988, t26993) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1859(t25049, t25277, t25077, t25080, t25140, t25144, t25293, t25317, t25211, t25346, t26198, t26200);
    (t26591, t26613, t26619, t26621, t26644, t26646, t26667, t26673, t26712, t26726, t26988, t26993)
}
