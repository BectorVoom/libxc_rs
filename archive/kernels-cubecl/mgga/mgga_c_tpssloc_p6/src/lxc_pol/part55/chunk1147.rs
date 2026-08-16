//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1147/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1147<F: Float>(t112943: F, t6562: F, t6572: F, t234: F, t6624: F, t22893: F, t23164: F, t30677: F, t23168: F, t30678: F, t30686: F, t6579: F) -> (F, F, F, F, F) {
    let t112948 = t6562 * t112943 * t6572;
    let t112951 = t234 * t6624;
    let t112961 = t23164 * t22893 * t30677;
    let t112968 = t23168 * t30678;
    let t112974 = t6579 * t30686;
    (t112948, t112951, t112961, t112968, t112974)
}
