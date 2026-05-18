//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1078/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1078<F: Float>(t2366: F, t25574: F, t1265: F, t986: F, t6508: F, t1352: F, t2755: F, t158: F, t7861: F, t1328: F, t20368: F, t2754: F, t475: F) -> (F, F, F, F, F, F, F, F) {
    let t25575 = t2366 * t25574;
    let t25579 = t986 * t1265;
    let t25580 = t6508 * t25579;
    let t25665 = t2755 * t1352;
    let t25694 = t158 * t7861;
    let t25722 = t986 * t1328;
    let t25723 = t20368 * t25722;
    let t25729 = t2754 * t475;
    (t25575, t25579, t25580, t25665, t25694, t25722, t25723, t25729)
}
