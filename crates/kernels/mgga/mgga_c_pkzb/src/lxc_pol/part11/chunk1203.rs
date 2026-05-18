//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1203/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1203<F: Float>(t29022: F, t29091: F, t29234: F, t29289: F, t29340: F, t29384: F, t29423: F, t29475: F, t1054: F, t3487: F, t2670: F, t3410: F) -> (F, F, F) {
    let t29478 = t29022 + t29091 + t29234 + t29289 + t29340 + t29384 + t29423 + t29475;
    let t29514 = t1054 * t3487;
    let t29562 = t2670 * t3410;
    (t29478, t29514, t29562)
}
