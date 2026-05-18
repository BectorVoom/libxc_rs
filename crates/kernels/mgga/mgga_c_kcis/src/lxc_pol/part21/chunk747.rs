//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 747/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk747<F: Float>(t752: F, t8532: F, t753: F, t124: F, t2491: F, t774: F) -> (F, F, F, F) {
    let t8533 = t752 * t8532;
    let t8535 = t753 * t753;
    let t8536 = F::new(1.0) / t8535;
    let t8537 = t124 * t8536;
    let t8538 = t2491 * t774;
    (t8533, t8536, t8537, t8538)
}
