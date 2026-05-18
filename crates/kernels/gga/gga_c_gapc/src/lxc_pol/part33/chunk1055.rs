//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 1055/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk1055<F: Float>(t33111: F, t687: F, t3721: F, t4905: F, t8601: F, t8616: F, t3179: F, t8598: F, t11706: F, t883: F, t2468: F, t3742: F) -> (F, F, F, F, F, F) {
    let t33113 = F::new(2.0) * t33111 * t687;
    let t33114 = t4905 * t3721;
    let t33116 = F::new(4.0) * t8601 * t8616;
    let t33119 = F::new(4.0) * t8598 * t3179;
    let t33121 = t11706 * t883;
    let t33129 = t3742 * t2468;
    (t33113, t33114, t33116, t33119, t33121, t33129)
}
