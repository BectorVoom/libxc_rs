//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2081/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2081<F: Float>(t91344: F, t26245: F, t80783: F, t22897: F, t6925: F, t26302: F, t80958: F, t22779: F, t26323: F, t1336: F, t242: F, t80901: F) -> (F, F, F, F, F, F) {
    let t91345 = F::cast_from(0.28260929265898273598e-2_f64) * t91344;
    let t91346 = t80783 * t26245;
    let t91351 = t6925 * t22897;
    let t91356 = t80958 * t26302;
    let t91357 = F::cast_from(0.16956557559538964159e-1_f64) * t91356;
    let t91358 = t22779 * t26323;
    let t91359 = F::cast_from(0.28260929265898273598e-2_f64) * t91358;
    let t91361 = t1336 * t80901 * t242;
    (t91345, t91346, t91351, t91357, t91359, t91361)
}
