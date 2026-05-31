//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 940/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk940<F: Float>(t13585: F, t5559: F, t841: F, t13578: F, t14537: F, t13346: F, t4342: F, t11305: F, t6556: F, t3599: F, t6553: F, t2595: F, t36313: F) -> (F, F, F, F, F, F) {
    let t45967 = F::cast_from(6.0_f64) * t5559 * t13585 * t841;
    let t45969 = F::cast_from(6.0_f64) * t14537 * t13578;
    let t45971 = F::cast_from(4.0_f64) * t4342 * t13346;
    let t45973 = F::cast_from(2.0_f64) * t6556 * t11305;
    let t45974 = t6553 * t3599;
    let t45976 = F::cast_from(2.0_f64) * t36313 * t2595;
    (t45967, t45969, t45971, t45973, t45974, t45976)
}
