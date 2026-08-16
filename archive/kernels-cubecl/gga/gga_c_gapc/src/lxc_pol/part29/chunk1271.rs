//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 1271/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk1271<F: Float>(t2911: F, t5918: F, t999: F, t11254: F, t2933: F, t3652: F, t8347: F, t11239: F, t8316: F, t11243: F, t8493: F, t190: F, t5589: F, t674: F, t8451: F) -> (F, F, F, F, F, F) {
    let t35668 = t2911 * t999 * t5918;
    let t35670 = t2933 * t11254;
    let t35672 = t8347 * t3652;
    let t35674 = t8316 * t11239;
    let t35676 = t8493 * t11243;
    let t35680 = t8451 * t190 * t674 * t5589;
    (t35668, t35670, t35672, t35674, t35676, t35680)
}
