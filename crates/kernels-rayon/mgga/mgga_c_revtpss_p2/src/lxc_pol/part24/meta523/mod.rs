//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta523 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1553;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1554;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta523(t1261: f64, t12884: f64, t24232: f64, t247: f64, t1263: f64, t24616: f64, t24633: f64, t17525: f64, t21188: f64, t24758: f64, t3172: f64, t3711: f64, t24643: f64, t24770: f64, t3153: f64, t17569: f64, t20783: f64, t1222: f64, t140: f64, t24816: f64, t24820: f64, t12915: f64, t24713: f64, t5384: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t82757, t82799, t82816, t82821, t82824) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1553(t1261, t12884, t24232, t247, t1263, t24616, t24633, t17525, t21188, t24758, t3172, t3711);
        let (t82827, t82859, t82932, t82980, t82983, t83014) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1554(t1261, t24643, t3172, t24770, t3153, t17569, t20783, t1222, t140, t24816, t24820, t12915, t247, t24713, t5384);
    (t82757, t82799, t82816, t82821, t82824, t82827, t82859, t82932, t82980, t82983, t83014)
}
