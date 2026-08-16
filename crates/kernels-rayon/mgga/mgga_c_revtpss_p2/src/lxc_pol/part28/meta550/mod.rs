//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta550 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1999;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2000;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta550(t1455: f64, t7337: f64, t2045: f64, t4153: f64, t10301: f64, t607: f64, t1927: f64, t2248: f64, t1926: f64, t25163: f64, t6973: f64, t644: f64, t6977: f64, t2315: f64, t2247: f64, t2259: f64, t2269: f64, t48: f64, t2275: f64, t613: f64, t10355: f64, t43: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t92559, t92563, t92565, t92570, t92573, t92576) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1999(t1455, t7337, t2045, t4153, t10301, t607, t1927, t2248, t1926, t25163, t6973, t644, t6977);
        let (t92577, t92585, t92588, t92597, t92600, t92605) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2000(t1926, t92576, t1927, t2315, t2247, t2259, t2269, t48, t2275, t613, t10355, t43);
    (t92559, t92563, t92565, t92570, t92573, t92577, t92585, t92588, t92597, t92600, t92605)
}
