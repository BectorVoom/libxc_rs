//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta353 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1282;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta353(t1178: f64, t3519: f64, t439: f64, t3522: f64, t447: f64, t300: f64, t3488: f64, t3800: f64, t498: f64, t1204: f64, t1269: f64, t12295: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t12552, t12553, t12555, t12571, t12587, t12603, t12610) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1282(t1178, t3519, t439, t3522, t447, t300, t3488, t3800, t498, t1204, t1269, t12295);
    (t12552, t12553, t12555, t12571, t12587, t12603, t12610)
}
