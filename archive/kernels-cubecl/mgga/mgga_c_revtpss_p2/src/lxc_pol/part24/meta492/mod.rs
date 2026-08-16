//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta492 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1489;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1490;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta492<F: Float>(t6846: F, t9909: F, t1413: F, t46835: F, t74483: F, t22061: F, t9793: F, t9794: F, t22026: F, t46802: F, t46694: F, t6850: F, t22245: F, t808: F, t9736: F, t22236: F, t6884: F, t9741: F, t14104: F, t47856: F, t2439: F, t3895: F, t6896: F, t136: F, t2457: F, t47480: F, t6895: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t74585, t74638, t74641, t74677, t74682) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1489::<F>(t6846, t9909, t1413, t46835, t74483, t22061, t9793, t9794, t22026, t46802, t46694, t6850);
        let (t74711, t74714, t74717, t74733, t74757, t74770) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1490::<F>(t22245, t808, t9736, t22236, t6884, t9741, t14104, t47856, t2439, t3895, t6896, t136, t2457, t47480, t6895);
    (t74585, t74638, t74641, t74677, t74682, t74711, t74714, t74717, t74733, t74757, t74770)
}
