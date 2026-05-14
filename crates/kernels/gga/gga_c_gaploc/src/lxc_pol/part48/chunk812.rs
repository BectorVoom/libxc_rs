//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 812/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk812<F: Float>(t13585: F, t5559: F, t841: F, t13578: F, t14537: F, t13346: F, t4342: F, t11305: F, t6556: F, t3599: F, t6553: F, t2595: F, t36313: F, t10283: F, t2902: F, t16710: F) -> (F, F, F, F, F, F, F, F) {
    let t45967 = 6.0 * t5559 * t13585 * t841;
    let t45969 = 6.0 * t14537 * t13578;
    let t45971 = 4.0 * t4342 * t13346;
    let t45973 = 2.0 * t6556 * t11305;
    let t45974 = t6553 * t3599;
    let t45976 = 2.0 * t36313 * t2595;
    let t45978 = 2.0 * t10283 * t2902;
    let t45983 = 24.0 * t16710 * t13578 * t841;
    (t45967, t45969, t45971, t45973, t45974, t45976, t45978, t45983)
}
