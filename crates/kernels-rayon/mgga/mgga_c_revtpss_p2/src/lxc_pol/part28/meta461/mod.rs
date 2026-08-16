//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta461 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1759;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1760;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta461(t25138: f64, t72: f64, t1927: f64, t6973: f64, t6977: f64, t2311: f64, t76: f64, t1926: f64, t10298: f64, t38: f64, t10309: f64, t6957: f64, t2248: f64, t77: f64, t84: f64, t2247: f64, t607: f64, t644: f64, t1923: f64, t1928: f64, t25099: f64, t25102: f64, t25106: f64, t25110: f64, t25114: f64, t25117: f64, t25120: f64, t6954: f64, t6958: f64, t6960: f64, t6963: f64, t6974: f64, t6978: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25139, t25140, t25143, t25146, t25147, t25150, t25157) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1759(t25138, t72, t1927, t6973, t6977, t2311, t76, t1926, t10298, t38, t10309, t6957);
        let (t25159, t25162, t25163, t25164, t25167) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1760(t2248, t77, t84, t2247, t607, t1927, t644, t1926, t1923, t1928, t25099, t25102, t25106, t25110, t25114, t25117, t25120, t25140, t25143, t25147, t25150, t25157, t6954, t6958, t6960, t6963, t6974, t6978);
    (t25139, t25140, t25143, t25146, t25147, t25150, t25157, t25159, t25162, t25163, t25164, t25167)
}
