//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1444/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1444<F: Float>(t12667: F, t36092: F, t36093: F, t37317: F, t37318: F, t37323: F, t37324: F, t37325: F, t37327: F, t37328: F, t37330: F, t38531: F, t38532: F, t38534: F, t38842: F, t38843: F, t38844: F, t38891: F, t38893: F, t7: F) -> F {
    let tv4rho2sigma216 = -t36092 + t36093 + t37317 + t38531 - t37318 + t38532 + t38534 + t7 * (t38891 + t38893) - t38842 - t37323 + t37324 + F::cast_from(2.0_f64) * t12667 + t37325 - t38843 - t38844 - t37327 + t37328 - t37330;
    tv4rho2sigma216
}
