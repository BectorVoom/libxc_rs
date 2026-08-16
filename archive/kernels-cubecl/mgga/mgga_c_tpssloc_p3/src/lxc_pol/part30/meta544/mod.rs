//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta544 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1895;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta544<F: Float>(t25049: F, t25277: F, t25077: F, t25080: F, t25140: F, t25144: F, t25293: F, t25317: F, t25211: F, t25346: F, t26198: F, t26200: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t26591, t26613, t26619, t26621, t26644, t26646, t26667, t26673, t26712, t26726, t26988, t26993) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1895::<F>(t25049, t25277, t25077, t25080, t25140, t25144, t25293, t25317, t25211, t25346, t26198, t26200);
    (t26591, t26613, t26619, t26621, t26644, t26646, t26667, t26673, t26712, t26726, t26988, t26993)
}
