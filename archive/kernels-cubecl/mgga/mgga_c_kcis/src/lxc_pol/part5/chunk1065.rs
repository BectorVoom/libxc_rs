//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1065/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1065<F: Float>(t3820: F, t562: F, t2029: F, t318: F, t86: F, t238: F, t5992: F, t2026: F, t752: F, t3393: F, t5973: F, t5981: F, t8931: F) -> (F, F, F, F, F, F) {
    let t17627 = t562 * t3820;
    let t17641 = t86 * t318 * t2029;
    let t17645 = F::cast_from(0.53062222222222222222e-1_f64) * t86 * t238 * t5992;
    let t17676 = t752 * t2026;
    let t17685 = F::cast_from(0.35374814814814814814e-1_f64) * t3393 * t5973;
    let t17686 = t8931 * t5981;
    (t17627, t17641, t17645, t17676, t17685, t17686)
}
