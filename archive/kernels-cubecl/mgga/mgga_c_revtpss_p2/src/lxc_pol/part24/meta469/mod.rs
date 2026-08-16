//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta469 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1446;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1447;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta469<F: Float>(t18349: F, t2689: F, t124: F, t5977: F, t10760: F, t18409: F, t9794: F, t18414: F, t40799: F, t18418: F, t18643: F, t40731: F, t10744: F, t808: F, t40521: F, t40791: F, t5989: F, t10890: F, t5985: F, t40627: F, t61837: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t61924, t61956, t61981, t62012, t62015, t62029) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1446::<F>(t18349, t2689, t124, t5977, t10760, t18409, t9794, t18414, t40799, t18418, t18643, t40731);
        let (t62069, t62072, t62089, t62095, t62111) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1447::<F>(t10744, t18409, t808, t18414, t40521, t40791, t5989, t10890, t5985, t10760, t40627, t61837);
    (t61924, t61956, t61981, t62012, t62015, t62029, t62069, t62072, t62089, t62095, t62111)
}
