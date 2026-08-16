//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1031/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1031(t224: f64, t2427: f64, t6789: f64, t14: f64, t35384: f64, t173: f64, t30779: f64, t35409: f64, t7470: f64, t140943: f64, t35405: f64, t33445: f64) -> (f64, f64, f64, f64, f64) {
    let t150687 = t224 * t2427 * t6789;
    let t150688 = t35384 * t14;
    let t150694 = t30779 * t7470 * t173 * t35409;
    let t150696 = t140943 * t35405;
    let t150697 = t33445 * t150696;
    (t150687, t150688, t150694, t150696, t150697)
}
