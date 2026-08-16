//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta344 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1196;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1197;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta344(t1066: f64, t23485: f64, t247: f64, t1651: f64, t5819: f64, t4801: f64, t1042: f64, t1668: f64, t6305: f64, t373: f64, t11257: f64, t11506: f64, t23451: f64, t11509: f64, t981: f64, t23448: f64, t23450: f64, t23461: f64, t23463: f64, t23465: f64, t23469: f64, t23549: f64, t23552: f64, t23554: f64, t23556: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t23630, t23633, t23634, t23635, t23640) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1196(t1066, t23485, t247, t1651, t5819, t4801, t1042, t1668, t6305);
        let (t23641, t23642, t23643, t23649, t23651, t23652) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1197(t23640, t373, t11257, t1042, t11506, t23451, t11509, t981, t23448, t23450, t23461, t23463, t23465, t23469, t23549, t23552, t23554, t23556);
    (t23630, t23633, t23634, t23635, t23640, t23641, t23642, t23643, t23649, t23651, t23652)
}
