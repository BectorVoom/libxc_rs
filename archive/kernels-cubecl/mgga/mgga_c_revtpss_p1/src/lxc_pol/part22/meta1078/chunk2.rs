//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3862/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3862<F: Float>(t13847: F, t22016: F, t48731: F, t73731: F, t13804: F, t22046: F, t46416: F, t48514: F, t48516: F, t48518: F, t48527: F, t48529: F, t48531: F, t48536: F, t48540: F, t48544: F, t5673: F) -> F {
    let t74232 = t48731 * t13847 * t73731 * t22016;
    let t74234 = -F::cast_from(0.28582678745379824648e-4_f64) * t48514 + F::cast_from(0.1219527626469539185e-2_f64) * t48516 + F::cast_from(0.7558530601555998074e-1_f64) * t48518 + F::cast_from(0.10164000561857065645e-2_f64) * t48527 + F::cast_from(0.1219527626469539185e-2_f64) * t48529 - F::cast_from(0.30488190661738479624e-3_f64) * t48531 - F::cast_from(0.11433071498151929859e-3_f64) * t48536 - F::cast_from(0.57165357490759649296e-4_f64) * t48540 + F::cast_from(0.28582678745379824648e-4_f64) * t48544 - F::cast_from(0.12862205435420921092e-2_f64) * t13804 * t5673 * t22046 * t46416 - F::cast_from(0.15246000842785598468e-3_f64) * t74232;
    t74234
}
