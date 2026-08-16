//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 878/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk878<F: Float>(t645: F, t8513: F, t8824: F, t31: F, t63: F, t607: F, t8308: F, t79: F, t641: F, t625: F, t8307: F, t8663: F) -> (F, F, F, F, F, F, F, F) {
    let t32328 = t8513 * t8824 * t645;
    let t32331 = t63 * t31;
    let t32332 = t32331 * t607;
    let t32333 = t8308 * t32332;
    let t32338 = t79 * t63;
    let t32340 = t8513 * t32338 * t641;
    let t32343 = t8307 * t625;
    let t32344 = t8513 * t32343;
    let t32346 = F::cast_from(5.0_f64) / F::cast_from(27.0_f64) * t8663 * t32344;
    (t32328, t32331, t32333, t32338, t32340, t32343, t32344, t32346)
}
