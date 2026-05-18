//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 1075/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk1075<F: Float>(t4043: F, t519: F, t1084: F, t9419: F, t11791: F, t3382: F, t129: F, t18551: F, t18553: F, t3284: F, t1086: F, t11311: F, t23466: F, t7624: F) -> (F, F, F, F, F, F) {
    let t33311 = t519 * t4043;
    let t33312 = t1084 * t33311;
    let t33313 = t33312 * t9419;
    let t33315 = t3382 * t11791;
    let t33320 = t18551 * t129 * t3284 * t18553;
    let t33324 = t7624 * t11311 * t1086 * t23466;
    (t33311, t33312, t33313, t33315, t33320, t33324)
}
