//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta461 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1759;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1760;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta461<F: Float>(t25138: F, t72: F, t1927: F, t6973: F, t6977: F, t2311: F, t76: F, t1926: F, t10298: F, t38: F, t10309: F, t6957: F, t2248: F, t77: F, t84: F, t2247: F, t607: F, t644: F, t1923: F, t1928: F, t25099: F, t25102: F, t25106: F, t25110: F, t25114: F, t25117: F, t25120: F, t6954: F, t6958: F, t6960: F, t6963: F, t6974: F, t6978: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t25139, t25140, t25143, t25146, t25147, t25150, t25157) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1759::<F>(t25138, t72, t1927, t6973, t6977, t2311, t76, t1926, t10298, t38, t10309, t6957);
        let (t25159, t25162, t25163, t25164, t25167) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1760::<F>(t2248, t77, t84, t2247, t607, t1927, t644, t1926, t1923, t1928, t25099, t25102, t25106, t25110, t25114, t25117, t25120, t25140, t25143, t25147, t25150, t25157, t6954, t6958, t6960, t6963, t6974, t6978);
    (t25139, t25140, t25143, t25146, t25147, t25150, t25157, t25159, t25162, t25163, t25164, t25167)
}
