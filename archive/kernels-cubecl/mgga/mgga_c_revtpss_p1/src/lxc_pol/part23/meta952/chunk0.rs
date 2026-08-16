//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3155/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3155<F: Float>(t5245: F, t6622: F, t1250: F, t16714: F, t17396: F, t17736: F, t17747: F, t19680: F, t20802: F, t20950: F, t20956: F, t21014: F, t21259: F, t21310: F, t3626: F, t3718: F, t3720: F, t44225: F, t44609: F, t5230: F, t5341: F, t5352: F, t57005: F, t57040: F, t59011: F, t6429: F, t6690: F, t72002: F, t82725: F, t82881: F, t82886: F) -> (F, F) {
    let t82899 = t5245 * t6622;
    let t82904 = F::cast_from(0.91464571985215438872e-2_f64) * t72002 * t21310 - F::cast_from(0.21437009059034868486e-3_f64) * t3718 * t3720 * t82725 * t5352 - F::cast_from(0.85748036236139473944e-3_f64) * t17736 * t3626 * t6429 * t5230 - F::cast_from(0.19055119163586549766e-2_f64) * t57005 * t44225 * t16714 * t19680 - F::cast_from(0.68598428988911579154e-2_f64) * t21014 * t20802 - F::cast_from(0.12862205435420921092e-2_f64) * t44609 * t3720 * t82881 * t1250 + F::cast_from(0.30011812682648815881e-2_f64) * t59011 * t3720 * t82886 * t5341 + F::cast_from(0.68598428988911579154e-2_f64) * t17396 * t21259 - F::cast_from(0.38586616306262763276e-2_f64) * t17747 * t3720 * t20956 * t20950 - F::cast_from(0.12862205435420921092e-2_f64) * t57040 * t6690 - F::cast_from(0.64311027177104605458e-3_f64) * t3718 * t3720 * t82899 * t1250;
    (t82899, t82904)
}
