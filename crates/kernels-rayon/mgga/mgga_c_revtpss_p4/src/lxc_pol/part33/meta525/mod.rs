//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta525 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1874;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1875;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta525(t27879: f64, t27907: f64, t27984: f64, t28017: f64, t532: f64, t1450: f64, t2014: f64, t1513: f64, t25823: f64, t665: f64, t25826: f64, t4287: f64, t6998: f64, t114: f64, t25822: f64, t25824: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t28019, t28020, t28021, t28022, t28034, t28036, t28037, t28039) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1874(t27879, t27907, t27984, t28017, t532, t1450, t2014, t1513, t25823, t665, t25826, t4287, t6998);
        let t28042 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1875(t114, t25822, t25824, t28034, t28037, t28039);
    (t28019, t28020, t28021, t28022, t28034, t28036, t28042)
}
