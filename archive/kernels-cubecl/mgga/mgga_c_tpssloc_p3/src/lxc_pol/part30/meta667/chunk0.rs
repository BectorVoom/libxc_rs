//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2093/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2093<F: Float>(t5259: F, t80820: F, t22779: F, t26292: F, t16060: F, t6944: F, t1827: F, t80991: F, t22765: F, t5289: F, t22764: F, t5234: F) -> (F, F, F, F, F, F) {
    let t91214 = t80820 * t5259;
    let t91215 = F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t91214;
    let t91225 = t22779 * t26292;
    let t91226 = F::cast_from(0.28260929265898273598e-2_f64) * t91225;
    let t91278 = t16060 * t6944;
    let t91281 = t80991 * t1827;
    let t91282 = F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t91281;
    let t91283 = t22765 * t5289;
    let t91284 = F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t91283;
    let t91285 = t5234 * t22764;
    (t91215, t91226, t91278, t91282, t91284, t91285)
}
