//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 988/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk988<F: Float>(t11109: F, t7810: F, t10628: F, t4820: F, t7513: F, t1029: F, t2617: F, t7803: F, t1052: F, t7822: F, t2972: F, t7324: F) -> (F, F, F, F, F, F, F) {
    let t11110 = t7810 * t11109;
    let t11111 = F::cast_from(0.19171462976960374838e0_f64) * t11110;
    let t11116 = t4820 * t10628;
    let t11118 = F::cast_from(0.79445533226334281487e-1_f64) * t7513 * t11116;
    let t11119 = t1029 * t2617;
    let t11120 = t7803 * t11119;
    let t11121 = F::cast_from(0.19171462976960374838e0_f64) * t11120;
    let t11130 = t7822 * t1052;
    let t11132 = F::new(2.0) * t7324 * t2972;
    (t11111, t11116, t11118, t11119, t11121, t11130, t11132)
}
