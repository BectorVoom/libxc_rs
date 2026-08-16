//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta297 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1056;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1057;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta297(t2664: f64, t9794: f64, t10760: f64, t2475: f64, t72: f64, t245: f64, t2482: f64, t814: f64, t823: f64, t136: f64, t853: f64, t220: f64, t124: f64, t836: f64, t2749: f64, t820: f64, t844: f64, t2751: f64, t2681: f64, t839: f64, t222: f64, t9727: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10762, t10769, t10770, t10777, t10779) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1056(t2664, t9794, t10760, t2475, t72, t245, t2482, t814, t823, t136, t853, t220);
        let (t10783, t10811, t10812, t10815, t10816, t10824) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1057(t124, t836, t10779, t2749, t10777, t820, t823, t844, t2751, t2681, t839, t222, t9727);
    (t10762, t10769, t10770, t10777, t10779, t10783, t10811, t10812, t10815, t10816, t10824)
}
