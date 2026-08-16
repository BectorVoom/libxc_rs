//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1083/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1083<F: Float>(t12907: F, t13475: F, t13483: F, t13491: F, t2: F, t873: F, t584: F, t265: F, t16: F, t4331: F, t10723: F, t4496: F) -> (F, F, F, F, F) {
    let t13493 = t12907 + t13475 + t13483 + t13491;
    let t13501 = t873 * t2;
    let t13503 = F::cast_from(2.0_f64) * t13501 * t584;
    let t13504 = t265 * t584;
    let t13506 = F::cast_from(3.0_f64) * t4331 * t16;
    let t13508 = t4496 * t10723;
    (t13493, t13503, t13504, t13506, t13508)
}
