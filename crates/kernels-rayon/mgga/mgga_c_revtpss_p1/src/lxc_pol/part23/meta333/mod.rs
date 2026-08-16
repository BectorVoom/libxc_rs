//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta333 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1633;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta333(t1437: f64, t1882: f64, t2482: f64, t4104: f64, t10073: f64, t5737: f64, t1419: f64, t4086: f64, t543: f64, t2782: f64, t555: f64, t5658: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14113, t14114, t14116, t14120, t14122, t14124, t14126, t14127) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1633(t1437, t1882, t2482, t4104, t10073, t5737, t1419, t4086, t543, t2782, t555, t5658);
    (t14113, t14114, t14116, t14120, t14122, t14124, t14126, t14127)
}
