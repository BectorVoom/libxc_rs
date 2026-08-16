//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1264/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1264(t30657: f64, t6547: f64, t30671: f64, t23030: f64, t30660: f64, t23204: f64, t30656: f64, t6562: f64, t30624: f64, t81591: f64, t30635: f64, t6579: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t112667 = t6547 * t30657;
    let t112673 = t6547 * t30671;
    let t112676 = 0.52089578783527170489e-1_f64 * t23030 * t30660;
    let t112678 = t6562 * t23204 * t30656;
    let t112680 = t81591 * t30624;
    let t112686 = t6579 * t30635;
    (t112667, t112673, t112676, t112678, t112680, t112686)
}
