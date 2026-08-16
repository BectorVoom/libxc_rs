//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1213/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1213(t214: f64, t33245: f64, t1985: f64, t1842: f64, t31558: f64, t22635: f64, t1992: f64, t1807: f64, t8617: f64, t31576: f64, t31578: f64, t31582: f64, t32712: f64, t32715: f64, t32718: f64, t32722: f64, t32724: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t33246 = t214 * t33245;
    let t33247 = t1985 * t33246;
    let t33249 = t31558 * t1842;
    let t33250 = t22635 * t33249;
    let t33251 = t1992 * t33250;
    let t33259 = t1807 * t8617;
    let t33266 = -t31576 - 0.96894614625936938046e-2_f64 * t32712 - t31578 - 0.16149102437656156341e-2_f64 * t32715 + t32718 / 768.0_f64 - t32722 / 768.0_f64 - t31582 - t32724 / 192.0_f64;
    (t33246, t33247, t33249, t33250, t33251, t33259, t33266)
}
