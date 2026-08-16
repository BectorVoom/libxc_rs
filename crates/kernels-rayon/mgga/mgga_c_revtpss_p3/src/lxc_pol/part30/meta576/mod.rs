//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta576 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2026;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta576(t94395: f64, t94398: f64, t4057: f64, t676: f64, t25880: f64, t25904: f64, t25945: f64, t9285: f64, t25944: f64, t1364: f64, t26075: f64, t786: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t94399, t94404, t94405, t94407, t94409, t94411) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2026(t94395, t94398, t4057, t676, t25880, t25904, t25945, t9285, t25944, t1364, t26075, t786);
    (t94399, t94404, t94405, t94407, t94409, t94411)
}
