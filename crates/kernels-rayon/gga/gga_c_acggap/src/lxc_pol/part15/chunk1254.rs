//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1254/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1254(t35835: f64, t37739: f64, t37741: f64, t40196: f64, t40200: f64, t40204: f64, t40208: f64, t40212: f64, t40218: f64, t40220: f64, t40222: f64, t40226: f64, t40230: f64, t40234: f64, t40237: f64, t40241: f64, t40243: f64, t40246: f64) -> f64 {
    let t42006 = 0.62896184579208304137e-2_f64 * t40196 + 0.62896184579208304137e-2_f64 * t40200 + 0.41930789719472202758e-2_f64 * t40204 - 0.21437009059034868486e-3_f64 * t40208 - 0.64311027177104605458e-2_f64 * t40212 + 0.18868855373762491241e-2_f64 * t40218 - 0.31448092289604152068e-2_f64 * t40220 - 0.51448821741683684368e-2_f64 * t35835 + t37739 - 0.12862205435420921092e-2_f64 * t40222 + 0.62896184579208304138e-3_f64 * t40226 + 0.62896184579208304138e-3_f64 * t40230 + 0.31448092289604152069e-3_f64 * t40234 + t37741 + 0.42874018118069736972e-3_f64 * t40237 - 0.94344276868812456207e-3_f64 * t40241 - 0.34299214494455789578e-1_f64 * t40243 + 0.21437009059034868486e-2_f64 * t40246;
    t42006
}
