//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 209/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk209(t40: f64, t52: f64, t761: f64, t763: f64, t201: f64, t262: f64, t73: f64, t607: f64, t76: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t146 = t40 <= zeta_threshold;
    let t150 = t52 <= zeta_threshold;
    let t765 = 0.5848223622634646207e0_f64 * t761 * t763;
    let t766 = t201 * t262;
    let t767 = 1.0_f64 / t73;
    let t770 = piecewise3(t146, 0.0_f64, 2.0_f64 / 3.0_f64 * t767 * t607);
    let t771 = 1.0_f64 / t76;
    let t774 = piecewise3(t150, 0.0_f64, -2.0_f64 / 3.0_f64 * t771 * t607);
    let t776 = t770 / 2.0_f64 + t774 / 2.0_f64;
    (t765, t766, t767, t771, t776)
}
