//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1313/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1313<F: Float>(t11239: F, t8316: F, t11243: F, t8493: F, t190: F, t5589: F, t674: F, t8451: F, t11395: F, t5: F, t25708: F, t4055: F, t8452: F) -> (F, F, F, F, F) {
    let t35674 = t8316 * t11239;
    let t35676 = t8493 * t11243;
    let t35680 = t8451 * t190 * t674 * t5589;
    let t35682 = t5 * t11395;
    let t35685 = t35682 * t25708 * t8452 * t4055;
    (t35674, t35676, t35680, t35682, t35685)
}
