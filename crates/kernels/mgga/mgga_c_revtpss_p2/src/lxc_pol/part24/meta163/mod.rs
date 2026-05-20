//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta163 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk811;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk812;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk813;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta163<F: Float>(t1150: F, t6470: F, t1131: F, t3435: F, t6438: F, t3433: F, t3439: F, t5044: F, t6423: F, t6427: F, t6431: F, t1744: F, t1169: F, t3459: F, t3466: F, t5093: F, t6443: F, t6450: F, t6456: F, t6458: F, t6462: F, t6465: F, t6468: F, t3479: F, t3483: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t6471, t6473, t6474, t6476, t6481, t6486) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk811::<F>(t1150, t6470, t1131, t3435, t6438, t3433, t3439, t5044, t6423, t6427, t6431, t1744);
        let (t6487, t6502) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk812::<F>(t1169, t6486, t3459, t3466, t5044, t5093, t6423, t6427, t6431, t6443, t6450, t6456, t6458, t6462, t6465, t6468);
        let (t6503, t6506, t6513) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk813::<F>(t1169, t6502, t3479, t6486, t3483, t5044, t6423, t6427, t6431);
    (t6471, t6473, t6474, t6476, t6481, t6486, t6487, t6502, t6503, t6506, t6513)
}
