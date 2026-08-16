//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta916 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2955;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2956;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta916(t15547: f64, t6223: f64, t1642: f64, t64510: f64, t23453: f64, t3022: f64, t1100: f64, t23571: f64, t41937: f64, t5023: f64, t77634: f64, t77636: f64, t77639: f64, t77641: f64, t77643: f64, t77645: f64, t77647: f64, t19082: f64, t4719: f64, t6219: f64, t6205: f64, t972: f64, t1634: f64, t52877: f64, t6227: f64, t23694: f64, t3011: f64, t4733: f64, t981: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t78405, t78411, t78413, t78414) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2955(t15547, t6223, t1642, t64510, t23453, t3022, t1100, t23571, t41937, t5023, t77634, t77636, t77639, t77641, t77643, t77645, t77647);
        let (t78417, t78422, t78423, t78426, t78428, t78432) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2956(t19082, t4719, t15547, t6219, t6205, t972, t1634, t52877, t6227, t23694, t3011, t4733, t981);
    (t78405, t78411, t78413, t78414, t78417, t78422, t78423, t78426, t78428, t78432)
}
