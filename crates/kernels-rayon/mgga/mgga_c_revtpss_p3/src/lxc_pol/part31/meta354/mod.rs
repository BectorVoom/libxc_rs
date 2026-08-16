//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta354 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1369;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1370;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta354(t14100: f64, t3917: f64, t136: f64, t1903: f64, t2457: f64, t9674: f64, t10175: f64, t5722: f64, t122: f64, t5721: f64, t3916: f64, t9680: f64, t1437: f64, t1882: f64, t2482: f64, t4104: f64, t10073: f64, t5737: f64, t1419: f64, t4086: f64, t543: f64, t2782: f64, t555: f64, t5658: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14102, t14104, t14105, t14108, t14109, t14110, t14111) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1369(t14100, t3917, t136, t1903, t2457, t9674, t10175, t5722, t122, t5721, t3916, t9680);
        let (t14116, t14120, t14126, t14127) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1370(t1437, t1882, t2482, t4104, t10073, t5737, t1419, t4086, t543, t2782, t555, t5658);
    (t14102, t14104, t14105, t14108, t14109, t14110, t14111, t14116, t14120, t14126, t14127)
}
