//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 945/1125 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk945<F: Float>(t1736: F, t435: F, t1084: F, t3375: F, t11512: F, t3707: F, t7375: F, t4043: F, t519: F, t9419: F, t11791: F, t3382: F, t129: F, t18551: F, t18553: F, t3284: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t33303 = t435 * t1736;
    let t33304 = t1084 * t33303;
    let t33305 = t33304 * t3375;
    let t33307 = t11512 * t3707;
    let t33309 = t1084 * t33307 * t7375;
    let t33311 = t519 * t4043;
    let t33312 = t1084 * t33311;
    let t33313 = t33312 * t9419;
    let t33315 = t3382 * t11791;
    let t33320 = t18551 * t129 * t3284 * t18553;
    (t33303, t33304, t33305, t33307, t33309, t33311, t33312, t33313, t33315, t33320)
}
