//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta540 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1587;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1588;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta540(t22912: f64, t4101: f64, t686: f64, t72: f64, t543: f64, t85659: f64, t2782: f64, t4100: f64, t4003: f64, t5744: f64, t86445: f64, t4086: f64, t86441: f64, t1904: f64, t22445: f64, t689: f64, t22974: f64, t47603: f64, t213: f64, t22964: f64, t13729: f64, t556: f64, t6918: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t86639, t86643, t86647, t86654) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1587(t22912, t4101, t686, t72, t543, t85659, t2782, t4100, t4003, t5744, t86445, t4086, t86441);
        let (t86682, t86699, t86701, t86712) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1588(t1904, t22445, t689, t22974, t47603, t686, t72, t213, t22964, t13729, t2782, t556, t6918);
    (t86639, t86643, t86647, t86654, t86682, t86699, t86701, t86712)
}
