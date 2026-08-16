//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 784/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk784(t13749: f64, t493: f64, t492: f64, t105: f64, t169: f64, t172: f64, t452: f64, t12771: f64, t12799: f64, t12805: f64, t12812: f64, t12821: f64, t12823: f64, t12824: f64, t12825: f64, t12828: f64, t12829: f64, t12832: f64, t12833: f64) -> (f64, f64, f64, f64, f64) {
    let t13750 = t493 * t13749;
    let t13751 = t492 * t13750;
    let t13753 = 0.28455006635676149599e-1_f64 * t105 * t13751;
    let t13755 = t13749 * t169 * t172;
    let t13756 = t452 * t13755;
    let t13758 = 0.28455006635676149599e-1_f64 * t105 * t13756;
    let t13759 = t12812 + t12828 + 0.11856252764865062333e-2_f64 * t12771 - 0.11856252764865062333e-2_f64 * t12821 - t13753 + t13758 + t12829 - t12833 - t12823 + t12824 + t12825 + t12799 + t12805 - t12832;
    (t13750, t13751, t13755, t13756, t13759)
}
