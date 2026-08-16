//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta447 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1868;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta447(t1634: f64, t4707: f64, t6209: f64, t972: f64, t6206: f64, t3014: f64, t6205: f64, t4711: f64, t11509: f64, t6189: f64, t15101: f64, t4595: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19294, t19297, t19300, t19303, t19304, t19307, t19310, t19311, t19315) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1868(t1634, t4707, t6209, t972, t6206, t3014, t6205, t4711, t11509, t6189, t15101, t4595);
    (t19294, t19297, t19300, t19303, t19304, t19307, t19310, t19311, t19315)
}
