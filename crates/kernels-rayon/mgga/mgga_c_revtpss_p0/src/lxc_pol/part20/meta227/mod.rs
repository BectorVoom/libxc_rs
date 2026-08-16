//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta227 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1018;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta227(t240: f64, t2719: f64, t243: f64, t2722: f64, t2723: f64, t2661: f64, t231: f64, t2662: f64, t10489: f64, t828: f64, t855: f64, t221: f64, t2430: f64, t2675: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10726, t10728, t10729, t10730, t10732, t10733, t10734, t10737, t10741) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1018(t240, t2719, t243, t2722, t2723, t2661, t231, t2662, t10489, t828, t855, t221, t2430, t2675);
    (t10726, t10728, t10729, t10730, t10732, t10733, t10734, t10737, t10741)
}
