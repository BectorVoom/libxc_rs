//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3128/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3128<F: Float>(t15339: F, t15376: F, t15419: F, t18232: F, t3447: F, t11593: F, t15317: F, t18427: F, t52019: F, t52022: F, t52038: F, t52050: F, t52053: F, t52057: F, t52061: F, t52064: F) -> F {
    let t64730 = t15376 * t15339;
    let t64733 = t3447 * t15419 * t18232;
    let t64746 = F::cast_from(0.59259259259259259256e-2_f64) * t15376 * t15317 - F::cast_from(0.987654320987654321e-3_f64) * t64730 + F::cast_from(0.24691358024691358024e-3_f64) * t64733 + F::cast_from(0.74074074074074074072e-3_f64) * t52019 - F::cast_from(0.49382716049382716048e-3_f64) * t52022 + F::cast_from(0.37037037037037037036e-3_f64) * t52038 + F::cast_from(0.24691358024691358024e-3_f64) * t52050 + F::cast_from(0.37037037037037037036e-3_f64) * t52053 + F::cast_from(0.49382716049382716048e-3_f64) * t52057 - F::cast_from(0.24691358024691358024e-3_f64) * t52061 + F::cast_from(0.49382716049382716048e-3_f64) * t52064 + F::cast_from(0.55555555555555555554e-3_f64) * t3447 * t11593 * t18427;
    t64746
}
