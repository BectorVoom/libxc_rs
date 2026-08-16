//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2873/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2873(t10702: f64, t2793: f64, t5730: f64, t13654: f64, t1557: f64, t2792: f64, t10661: f64, t2836: f64, t17527: f64, t42028: f64, t41831: f64, t47705: f64, t47707: f64, t47709: f64, t47711: f64, t47713: f64, t47715: f64, t47717: f64, t47724: f64, t47730: f64, t47732: f64, t48087: f64, t48096: f64, t48098: f64) -> (f64, f64, f64, f64, f64) {
    let t60047 = 0.57895126195293126241e3_f64 * t10702 * t5730 * t2793;
    let t60050 = 4.0_f64 * t2792 * t1557 * t13654;
    let t60053 = 0.96491876992155210402e2_f64 * t10661 * t5730 * t2836;
    let t60056 = 0.62071215503128080361e4_f64 * t42028 * t17527 * t2793;
    let t60079 = 0.66228e0_f64 * t48087 + 0.10735111111111111112e1_f64 * t47705 - 0.35783703703703703705e0_f64 * t47707 + 0.26837777777777777778e0_f64 * t47709 + 0.13418888888888888889e0_f64 * t47711 + 0.22364814814814814815e0_f64 * t47713 - 0.80513333333333333336e0_f64 * t47715 - 0.40256666666666666668e0_f64 * t47717 - 0.80513333333333333335e0_f64 * t47724 + 0.18396666666666666667e0_f64 * t41831 - 0.36793333333333333334e0_f64 * t48096 + 0.11038e0_f64 * t48098 - 0.53675555555555555558e0_f64 * t47730 + 0.20128333333333333334e0_f64 * t47732;
    (t60047, t60050, t60053, t60056, t60079)
}
