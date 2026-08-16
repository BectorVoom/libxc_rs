//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta492 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1489;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1490;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta492(t6846: f64, t9909: f64, t1413: f64, t46835: f64, t74483: f64, t22061: f64, t9793: f64, t9794: f64, t22026: f64, t46802: f64, t46694: f64, t6850: f64, t22245: f64, t808: f64, t9736: f64, t22236: f64, t6884: f64, t9741: f64, t14104: f64, t47856: f64, t2439: f64, t3895: f64, t6896: f64, t136: f64, t2457: f64, t47480: f64, t6895: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t74585, t74638, t74641, t74677, t74682) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1489(t6846, t9909, t1413, t46835, t74483, t22061, t9793, t9794, t22026, t46802, t46694, t6850);
        let (t74711, t74714, t74717, t74733, t74757, t74770) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1490(t22245, t808, t9736, t22236, t6884, t9741, t14104, t47856, t2439, t3895, t6896, t136, t2457, t47480, t6895);
    (t74585, t74638, t74641, t74677, t74682, t74711, t74714, t74717, t74733, t74757, t74770)
}
