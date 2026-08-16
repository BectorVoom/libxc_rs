//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta424 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1486;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta424(t5876: f64, t670: f64, t5891: f64, t665: f64, t1513: f64, t4287: f64, t5915: f64, t5920: f64, t648: f64, t21881: f64, t94: f64, t1518: f64, t4245: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t85360, t105872, t105875, t105880, t108710, t108714, t109150) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1486(t5876, t670, t5891, t665, t1513, t4287, t5915, t5920, t648, t21881, t94, t1518, t4245);
    (t85360, t105872, t105875, t105880, t108710, t108714, t109150)
}
