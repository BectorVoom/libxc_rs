//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta297 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1545;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1546;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta297<F: Float>(t760: F, t9323: F, t9318: F, t2251: F, t750: F, t2611: F, t10467: F, t162: F, t187: F, t2398: F, t2615: F, t2609: F, t717: F, t9544: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t10552, t10554, t10555, t10556, t10557, t10558, t10560, t10561, t10562, t10563) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1545::<F>(t760, t9323, t9318, t2251, t750, t2611, t10467, t162, t187, t2398, t2615, t2609, t717);
        let (t10564, t10565) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1546::<F>(t10563, t162, t9544);
    (t10552, t10554, t10555, t10556, t10557, t10558, t10560, t10561, t10562, t10563, t10564, t10565)
}
