//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 845/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk845(t8278: f64, t8291: f64, t8292: f64, t8294: f64, t8945: f64, t8953: f64, t8973: f64, t8975: f64, t8981: f64, t8983: f64, t9739: f64, t9741: f64, t9743: f64, t9747: f64, t9749: f64, t9751: f64, t9753: f64, t9755: f64, t9759: f64, t9762: f64) -> f64 {
    let t9968 = -t8278 - 7.0_f64 / 72.0_f64 * t8945 - 0.62896184579208304138e-3_f64 * t8953 - t9739 / 12.0_f64 - t9741 / 24.0_f64 + t9743 / 8.0_f64 + 0.12862205435420921092e-1_f64 * t8973 - 0.11321313224257494745e-1_f64 * t8975 - 0.18868855373762491241e-1_f64 * t8981 + 0.51448821741683684367e-2_f64 * t8983 - t9747 / 24.0_f64 - t9749 / 48.0_f64 + t9751 / 24.0_f64 + 0.17149607247227894789e-1_f64 * t9753 + 0.51448821741683684367e-2_f64 * t9755 + t8291 + t8292 - 0.21437009059034868486e-3_f64 * t9759 + t8294 + t9762 / 48.0_f64;
    t9968
}
