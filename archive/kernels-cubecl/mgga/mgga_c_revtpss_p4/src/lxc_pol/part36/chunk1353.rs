//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1353/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1353<F: Float>(t104682: F, t104685: F, t104825: F, t112328: F, t112334: F, t112336: F, t24649: F, t24731: F, t24736: F, t24741: F, t24804: F, t24808: F, t24836: F, t26867: F, t26880: F, t29010: F, t29097: F, t29100: F, t6625: F, t6631: F, t6635: F, t7624: F, t97149: F, t97179: F) -> F {
    let t116185 = -F::cast_from(0.11433071498151929859e-2_f64) * t112328 + F::cast_from(0.25724410870841842183e-2_f64) * t29097 * t24731 - F::cast_from(0.12862205435420921092e-2_f64) * t29100 * t24736 + F::cast_from(0.25724410870841842183e-2_f64) * t97179 * t24741 - F::cast_from(0.25724410870841842183e-2_f64) * t97149 * t24836 - F::cast_from(0.85748036236139473944e-3_f64) * t112334 + F::cast_from(0.17149607247227894789e-2_f64) * t112336 + F::cast_from(0.12862205435420921092e-2_f64) * t29010 * t6625 + F::cast_from(0.25724410870841842183e-2_f64) * t104682 * t6631 - F::cast_from(0.12862205435420921092e-2_f64) * t104685 * t6635 + F::cast_from(0.28582678745379824648e-3_f64) * t104825 + F::cast_from(0.85748036236139473944e-3_f64) * t26880 * t24649 - F::cast_from(0.17149607247227894789e-2_f64) * t7624 * t24808 + F::cast_from(0.14291339372689912324e-2_f64) * t26867 * t24804;
    t116185
}
