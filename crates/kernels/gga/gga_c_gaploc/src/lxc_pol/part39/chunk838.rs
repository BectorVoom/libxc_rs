//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 838/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk838<F: Float>(t1358: F, t3394: F, t488: F, t9060: F, t10171: F, t2317: F, t6525: F, t2321: F, t34478: F, t9074: F, t123: F, t31730: F, t2326: F, t12797: F, t12773: F, t6305: F) -> (F, F, F, F, F, F) {
    let t42659 = 0.31616674039640166221e-2 * t1358 * t9060 * t3394 * t488;
    let t42661 = t6525 * t10171 * t2317;
    let t42664 = t9074 * t34478 * t2321;
    let t42669 = t31730 * t123;
    let t42671 = t9074 * t42669 * t2326;
    let t42673 = t1358 * t12797;
    let t42674 = 0.31616674039640166221e-2 * t42673;
    let t42675 = t6305 * t12773;
    (t42659, t42661, t42664, t42671, t42674, t42675)
}
