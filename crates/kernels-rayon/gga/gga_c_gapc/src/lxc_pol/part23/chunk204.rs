//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 204/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk204(t408: f64, t411: f64, t414: f64, t419: f64, t88: f64, t257: f64, t260: f64, t266: f64, t738: f64, t748: f64, t751: f64) -> (f64, f64) {
    let t758 = 0.77371026992393176896e-2_f64 * t88 - 0.2499945e-2_f64 * t408 + 0.604634375e-3_f64 * t411 - 0.20417003743104289064e-4_f64 * t414 + 0.20205871875e-5_f64 * t419;
    let t760 = -0.10636476373080147432e-2_f64 * t88 * t257 - 0.21272952746160294864e-2_f64 * t738 * t748 - t751 * t266 - t260 * t758;
    (t758, t760)
}
