//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 667/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk667(t225: f64, t2711: f64, t2594: f64, t120: f64, t2678: f64, t2631: f64, t2592: f64, t252: f64, t856: f64, t68: f64, t261: f64, t2751: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9590 = t2711 * t225;
    let t9593 = t2594 * t225;
    let t9621 = t120 * t2678;
    let t9626 = t120 * t2631;
    let t10049 = t2592 * t225;
    let t10097 = t252 * t2678;
    let t10108 = t856 * t856;
    let t10109 = 1.0_f64 / t10108;
    let t10110 = t68 * t10109;
    let t10143 = 1.0_f64 / t2751 / t261;
    (t9590, t9593, t9621, t9626, t10049, t10097, t10108, t10109, t10110, t10143)
}
