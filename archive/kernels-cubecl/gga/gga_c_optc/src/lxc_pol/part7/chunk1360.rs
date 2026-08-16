//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1360/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1360<F: Float>(t2849: F, t3117: F, t429: F, t745: F, t116: F, t428: F, t1111: F, t1115: F, t1781: F, t24: F, t8483: F, t371: F) -> (F, F, F, F, F, F) {
    let t27067 = t3117 * t2849;
    let t27071 = t745 * t429;
    let t27074 = F::cast_from(5.0_f64) / F::cast_from(486.0_f64) * t428 * t116 * t27071;
    let t27076 = t1111 * t1781 * t1115;
    let t27079 = t1111 * t24 * t8483;
    let t27082 = F::cast_from(1.0_f64) / t371 / t2849;
    (t27067, t27071, t27074, t27076, t27079, t27082)
}
