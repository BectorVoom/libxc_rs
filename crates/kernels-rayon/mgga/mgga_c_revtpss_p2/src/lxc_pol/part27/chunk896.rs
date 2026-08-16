//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 896/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk896(t10818: f64, t2477: f64, t828: f64, t222: f64, t9727: f64, t2737: f64, t9802: f64, t10639: f64, t827: f64, t221: f64, t2485: f64, t2754: f64) -> (f64, f64, f64, f64, f64) {
    let t10820 = t2477 * t828 * t10818;
    let t10824 = 455.0_f64 / 1296.0_f64 * t9727 * t222;
    let t10826 = 0.45738002528356795401e-4_f64 * t9802 * t2737;
    let t10828 = t827 * t828 * t10639;
    let t10832 = t2485 * t221 * t2754;
    (t10820, t10824, t10826, t10828, t10832)
}
