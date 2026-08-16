//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 959/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk959(t120363: f64, t120375: f64, t120393: f64, t120416: f64, t2105: f64, t8110: f64, t112: f64, t34175: f64, t111: f64, t34136: f64, t1437: f64, t63: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t124142 = 0.5383034145885385447e-3_f64 * t120363;
    let t124146 = 7.0_f64 / 144.0_f64 * t120375;
    let t124154 = 0.32298204875312312682e-2_f64 * t120393;
    let t124163 = 7.0_f64 / 576.0_f64 * t120416;
    let t124673 = t8110 * t2105;
    let t124676 = t34175 * t112;
    let t124728 = t34136 * t111;
    let t124755 = t63 * t1437;
    (t124142, t124146, t124154, t124163, t124673, t124676, t124728, t124755)
}
