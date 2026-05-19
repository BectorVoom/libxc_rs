//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 923/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk923<F: Float>(t10525: F, t10526: F, t41878: F, t30829: F, t31769: F, t544: F, t913: F, t1424: F, t2875: F, t9060: F, t10405: F, t2478: F, t6583: F) -> (F, F, F, F) {
    let t41880 = t10525 * t10526 * t41878;
    let t41884 = t544 * t30829 * t913 * t31769;
    let t41885 = F::cast_from(0.3575048995185042667e0_f64) * t41884;
    let t41889 = F::cast_from(0.39722766613167140743e-1_f64) * t544 * t9060 * t2875 * t1424;
    let t41891 = t6583 * t10405 * t2478;
    (t41880, t41885, t41889, t41891)
}
