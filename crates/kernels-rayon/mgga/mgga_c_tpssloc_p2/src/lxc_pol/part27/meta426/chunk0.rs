//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1740/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1740(t22666: f64, t6907: f64, t1985: f64, t225: f64, t6956: f64, t562: f64, t794: f64) -> (f64, f64, f64, f64) {
    let t22667 = t22666 * t6907;
    let t22668 = t1985 * t22667;
    let t22670 = t6956 * t225;
    let t22674 = t794 * t562;
    (t22667, t22668, t22670, t22674)
}
