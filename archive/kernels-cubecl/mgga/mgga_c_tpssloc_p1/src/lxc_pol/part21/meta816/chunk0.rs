//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2873/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2873<F: Float>(t10702: F, t2793: F, t5730: F, t13654: F, t1557: F, t2792: F, t10661: F, t2836: F, t17527: F, t42028: F, t41831: F, t47705: F, t47707: F, t47709: F, t47711: F, t47713: F, t47715: F, t47717: F, t47724: F, t47730: F, t47732: F, t48087: F, t48096: F, t48098: F) -> (F, F, F, F, F) {
    let t60047 = F::cast_from(0.57895126195293126241e3_f64) * t10702 * t5730 * t2793;
    let t60050 = F::cast_from(4.0_f64) * t2792 * t1557 * t13654;
    let t60053 = F::cast_from(0.96491876992155210402e2_f64) * t10661 * t5730 * t2836;
    let t60056 = F::cast_from(0.62071215503128080361e4_f64) * t42028 * t17527 * t2793;
    let t60079 = F::cast_from(0.66228e0_f64) * t48087 + F::cast_from(0.10735111111111111112e1_f64) * t47705 - F::cast_from(0.35783703703703703705e0_f64) * t47707 + F::cast_from(0.26837777777777777778e0_f64) * t47709 + F::cast_from(0.13418888888888888889e0_f64) * t47711 + F::cast_from(0.22364814814814814815e0_f64) * t47713 - F::cast_from(0.80513333333333333336e0_f64) * t47715 - F::cast_from(0.40256666666666666668e0_f64) * t47717 - F::cast_from(0.80513333333333333335e0_f64) * t47724 + F::cast_from(0.18396666666666666667e0_f64) * t41831 - F::cast_from(0.36793333333333333334e0_f64) * t48096 + F::cast_from(0.11038e0_f64) * t48098 - F::cast_from(0.53675555555555555558e0_f64) * t47730 + F::cast_from(0.20128333333333333334e0_f64) * t47732;
    (t60047, t60050, t60053, t60056, t60079)
}
