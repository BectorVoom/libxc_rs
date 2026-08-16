//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1048/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1048(t3099: f64, t52: f64, t7182: f64, t7189: f64, t938: f64, t136866: f64, t6427: f64, t136968: f64, t934: f64, t25658: f64, t32296: f64, t115418: f64, t136996: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t145353 = t52 * t7182 * t3099;
    let t145361 = t7189 * t938;
    let t145372 = t136866 * t6427;
    let t145376 = t136968 * t934;
    let t145379 = t32296 * t25658;
    let t145382 = t136996 * t115418;
    (t145353, t145361, t145372, t145376, t145379, t145382)
}
