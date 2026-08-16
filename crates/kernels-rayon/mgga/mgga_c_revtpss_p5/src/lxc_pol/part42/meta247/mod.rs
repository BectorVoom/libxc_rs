//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta247 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk938;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta247(t225: f64, t494: f64, t6695: f64, t1828: f64, t3737: f64, t1280: f64, t6573: f64, t1287: f64, t6688: f64, t1774: f64, t5486: f64, t6587: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t6697, t6702, t6703, t6714, t6717, t6720, t6723) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk938(t225, t494, t6695, t1828, t3737, t1280, t6573, t1287, t6688, t1774, t5486, t6587);
    (t6697, t6702, t6703, t6714, t6717, t6720, t6723)
}
