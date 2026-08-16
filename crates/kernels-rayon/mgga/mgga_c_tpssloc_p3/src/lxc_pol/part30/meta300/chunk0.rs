//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1318/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1318(t10027: f64, t222: f64, t805: f64, t9541: f64, t2627: f64, t852: f64, t856: f64, t68: f64, t261: f64, t2751: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10029 = 455.0_f64 / 1296.0_f64 * t10027 * t222;
    let t10036 = t9541 * t805;
    let t10054 = t2627 * t852;
    let t10108 = t856 * t856;
    let t10109 = 1.0_f64 / t10108;
    let t10110 = t68 * t10109;
    let t10143 = 1.0_f64 / t2751 / t261;
    (t10029, t10036, t10054, t10108, t10109, t10110, t10143)
}
