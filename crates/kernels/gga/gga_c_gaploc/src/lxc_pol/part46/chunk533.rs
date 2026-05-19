//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 533/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk533<F: Float>(t6696: F, t901: F, t6700: F, t3162: F, t549: F, t1429: F, t2372: F, t2389: F, t9554: F, t9557: F, t9560: F, t9564: F, t9565: F, t9569: F, t9571: F, t9575: F) -> (F, F, F, F, F) {
    let t9577 = F::cast_from(0.29792074959875355558e-1_f64) * t6696 * t901;
    let t9579 = F::cast_from(0.29792074959875355558e-1_f64) * t6700 * t901;
    let t9580 = t549 * t3162;
    let t9582 = F::cast_from(0.59584149919750711116e-1_f64) * t1429 * t9580;
    let t9584 = F::cast_from(0.59584149919750711116e-1_f64) * t2372 * t2389;
    let t9585 = -t9554 + t9557 + t9560 - t9564 + F::cast_from(0.39722766613167140743e-1_f64) * t1429 * t9565 - t9569 - t9571 - t9575 + t9577 + t9579 + t9582 - t9584;
    (t9577, t9579, t9582, t9584, t9585)
}
