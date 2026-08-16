//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 585/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk585(t265: f64, t394: f64, t2057: f64, t25: f64, t202: f64, t2056: f64, t193: f64, t870: f64) -> (f64, f64, f64, f64) {
    let t395 = t265 < t394;
    let t2058 = t2057 * t25;
    let t2061 = t202 * t2056;
    let t2063 = t193 * t2061 * t870;
    let t2064 = piecewise3(t395, 0.0_f64, t2063);
    (t2058, t2061, t2063, t2064)
}
