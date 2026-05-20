//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta524 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2043;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2044;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta524<F: Float>(t1828: F, t5497: F, t3737: F, t1269: F, t6628: F, t3783: F, t3769: F, t1280: F, t20703: F, t1287: F, t1811: F, t5284: F, t1284: F, t6564: F) -> (F, F, F, F, F, F, F) {
        let (t21407, t21408, t21416, t21427, t21430, t21436) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2043::<F>(t1828, t5497, t3737, t1269, t6628, t3783, t3769, t1280, t20703, t1287, t1811, t5284);
        let t21439 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2044::<F>(t1284, t6564);
    (t21407, t21408, t21416, t21427, t21430, t21436, t21439)
}
