//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1034/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1034(t2712: f64, t64: f64, t2710: f64, t826: f64, t2482: f64, t27: f64, t7036: f64, t2487: f64, t2479: f64, t7045: f64, t2648: f64, t7038: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25240 = t64 * t2712;
    let t25242 = t2710 * t25240 * t826;
    let t25245 = t2482 * t7036 * t27;
    let t25246 = t25245 * t2487;
    let t25248 = t7045 * t2479;
    let t25251 = t7038 * t2648;
    (t25240, t25242, t25245, t25246, t25248, t25251)
}
