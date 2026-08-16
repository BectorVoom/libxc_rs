//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta856 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3001;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3002;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta856(t2439: f64, t4469: f64, t780: f64, t785: f64, t213: f64, t252: f64, t2440: f64, t4534: f64, t1580: f64, t41117: f64, t10509: f64, t10995: f64, t14990: f64, t122: f64, t14982: f64, t2466: f64, t10777: f64, t10779: f64, t1548: f64, t2646: f64, t10868: f64, t820: f64, t844: f64, t14896: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t50236, t50240, t50245, t50248, t50253) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3001(t2439, t4469, t780, t785, t213, t252, t2440, t4534, t1580, t41117, t10509, t10995, t14990);
        let (t50259, t50292, t50295, t50296) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3002(t10995, t122, t14982, t2466, t10777, t10779, t1548, t2646, t10868, t820, t844, t14896);
    (t50236, t50240, t50245, t50248, t50253, t50259, t50292, t50295, t50296)
}
