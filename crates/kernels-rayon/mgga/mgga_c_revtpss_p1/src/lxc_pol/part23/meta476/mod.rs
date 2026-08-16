//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta476 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1928;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta476(t1063: f64, t20054: f64, t19572: f64, t4894: f64, t3117: f64, t4900: f64, t11774: f64, t15926: f64, t20040: f64, t20046: f64, t20051: f64, t3106: f64, t3188: f64, t4892: f64, t4899: f64, t4912: f64, t6323: f64, t6327: f64, t6331: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t20055, t20065, t20066, t20069, t20070, t20073) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1928(t1063, t20054, t19572, t4894, t3117, t4900, t11774, t15926, t20040, t20046, t20051, t3106, t3188, t4892, t4899, t4912, t6323, t6327, t6331);
    (t20055, t20065, t20066, t20069, t20070, t20073)
}
