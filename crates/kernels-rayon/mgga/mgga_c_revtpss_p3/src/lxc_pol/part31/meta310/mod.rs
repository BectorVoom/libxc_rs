//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta310 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1308;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta310(t10760: f64, t10761: f64, t2475: f64, t72: f64, t245: f64, t2482: f64, t814: f64, t823: f64, t136: f64, t853: f64, t220: f64, t2723: f64, t775: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t10762, t10769, t10770, t10777, t10778, t10779, t10786) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1308(t10760, t10761, t2475, t72, t245, t2482, t814, t823, t136, t853, t220, t2723, t775);
    (t10762, t10769, t10770, t10777, t10778, t10779, t10786)
}
