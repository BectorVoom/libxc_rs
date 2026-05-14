//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1204/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1204<F: Float>(t35077: F, t35086: F, t37184: F, t37185: F, t37186: F, t37188: F, t37189: F, t37191: F, t37192: F, t37193: F, t37194: F, t35149: F, t37210: F, t37211: F, t37212: F, t37213: F, t37214: F, t37216: F, t37217: F, t37218: F, t37219: F, t37220: F) -> (F, F) {
    let t38650 = t37184 - t37185 - t37186 + 0.57970906942607043475e-5 * t35077 - t37188 - t37189 + 0.33460450185846399382e-7 * t35086 + t37191 - t37192 + t37193 + t37194;
    let t38661 = -t37210 - t37211 - t37212 - t37213 + t37214 - 0.64456181686737100546e-8 * t35149 + t37216 + t37217 + t37218 + t37219 - t37220;
    (t38650, t38661)
}
