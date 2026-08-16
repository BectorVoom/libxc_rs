//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta584 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1912;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1913;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta584(t10073: f64, t25937: f64, t7282: f64, t8085: f64, t102235: f64, t25904: f64, t102215: f64, t25878: f64, t102385: f64, t94383: f64, t102394: f64, t26260: f64, t27836: f64, t1385: f64, t1903: f64, t26304: f64, t28925: f64, t531: f64, t2411: f64, t28455: f64, t198: f64, t206: f64, t8019: f64, t28309: f64, t686: f64, t72: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t102610, t102615, t102617, t102629, t102634, t102636) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1912(t10073, t25937, t7282, t8085, t102235, t25904, t102215, t25878, t102385, t94383, t102394, t26260, t27836);
        let (t102656, t102661, t102769, t102854, t102888, t102928) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1913(t1385, t8085, t1903, t26304, t28925, t531, t2411, t28455, t198, t206, t8019, t28309, t686, t72);
    (t102610, t102615, t102617, t102629, t102634, t102636, t102656, t102661, t102769, t102854, t102888, t102928)
}
