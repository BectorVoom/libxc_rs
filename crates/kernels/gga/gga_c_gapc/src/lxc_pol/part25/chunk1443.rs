//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1443/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1443<F: Float>(t12589: F, t12623: F, t12588: F, t12624: F, t36092: F, t36093: F, t37317: F, t37318: F, t37323: F, t37324: F, t37325: F, t37327: F, t37328: F, t37330: F, t38531: F, t38532: F, t38534: F, t38693: F, t38839: F, t7: F) -> F {
    let t38842 = F::new(2.0) * t12589;
    let t38843 = F::new(2.0) * t12623;
    let t38844 = F::new(2.0) * t12588;
    let tv4rho2sigma24 = -t36092 + t36093 + t37317 + t38531 - t37318 + t38532 + F::new(2.0) * t12624 + t38534 + t7 * (t38693 + t38839) - t38842 - t37323 + t37324 + t37325 - t38843 - t38844 - t37327 + t37328 - t37330;
    tv4rho2sigma24
}
