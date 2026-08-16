//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 841/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk841<F: Float>(t9514: F, t9804: F, t105: F, t469: F, t182: F, t310: F, t129: F, t5: F, t2162: F, t814: F, t301: F, t624: F) -> (F, F, F, F, F, F, F) {
    let t9805 = t9514 + t9804;
    let t9806 = t105 * t9805;
    let t9807 = t9806 * t469;
    let t10098 = t310 * t182;
    let t10146 = t129 * t5;
    let t10409 = t814 * t2162;
    let t10586 = t624 * t301;
    (t9805, t9806, t9807, t10098, t10146, t10409, t10586)
}
