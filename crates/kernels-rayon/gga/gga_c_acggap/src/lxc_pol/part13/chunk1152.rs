//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1152/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1152(t17912: f64, t2288: f64, t31443: f64, t3169: f64, t31598: f64, t31602: f64, t35766: f64, t35768: f64, t35772: f64, t35775: f64, t35778: f64, t35782: f64, t35785: f64, t35789: f64, t35790: f64, t35792: f64, t35795: f64, t35798: f64, t35800: f64, t35801: f64, t35804: f64) -> f64 {
    let t35808 = t31443 * t17912 * t2288 * t3169;
    let t35810 = 0.68598428988911579156e-2_f64 * t35766 - 0.68598428988911579156e-2_f64 * t35768 - t31598 - t31602 + 0.64311027177104605458e-3_f64 * t35772 + t35775 + 0.21437009059034868486e-2_f64 * t35778 + 0.12862205435420921092e-2_f64 * t35782 + t35785 + t35789 + 0.85748036236139473944e-3_f64 * t35790 - 0.85748036236139473945e-2_f64 * t35792 - t35795 + t35798 + t35800 - 0.20579528696673473746e-1_f64 * t35801 + 0.47172138434406228102e-2_f64 * t35804 - 0.18868855373762491241e-2_f64 * t35808;
    t35810
}
