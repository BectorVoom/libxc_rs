//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3155/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3155(t5245: f64, t6622: f64, t1250: f64, t16714: f64, t17396: f64, t17736: f64, t17747: f64, t19680: f64, t20802: f64, t20950: f64, t20956: f64, t21014: f64, t21259: f64, t21310: f64, t3626: f64, t3718: f64, t3720: f64, t44225: f64, t44609: f64, t5230: f64, t5341: f64, t5352: f64, t57005: f64, t57040: f64, t59011: f64, t6429: f64, t6690: f64, t72002: f64, t82725: f64, t82881: f64, t82886: f64) -> (f64, f64) {
    let t82899 = t5245 * t6622;
    let t82904 = 0.91464571985215438872e-2_f64 * t72002 * t21310 - 0.21437009059034868486e-3_f64 * t3718 * t3720 * t82725 * t5352 - 0.85748036236139473944e-3_f64 * t17736 * t3626 * t6429 * t5230 - 0.19055119163586549766e-2_f64 * t57005 * t44225 * t16714 * t19680 - 0.68598428988911579154e-2_f64 * t21014 * t20802 - 0.12862205435420921092e-2_f64 * t44609 * t3720 * t82881 * t1250 + 0.30011812682648815881e-2_f64 * t59011 * t3720 * t82886 * t5341 + 0.68598428988911579154e-2_f64 * t17396 * t21259 - 0.38586616306262763276e-2_f64 * t17747 * t3720 * t20956 * t20950 - 0.12862205435420921092e-2_f64 * t57040 * t6690 - 0.64311027177104605458e-3_f64 * t3718 * t3720 * t82899 * t1250;
    (t82899, t82904)
}
