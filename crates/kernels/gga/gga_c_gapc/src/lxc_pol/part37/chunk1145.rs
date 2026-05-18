//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1145/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1145<F: Float>(t15542: F, t33287: F, t7953: F, t21801: F, t7259: F, t7325: F, t11799: F, t129: F, t18866: F, t11798: F, t28370: F, t7453: F) -> (F, F, F, F, F) {
    let t33289 = t7953 * t33287 * t15542;
    let t33291 = t7259 * t21801;
    let t33292 = t33291 * t7325;
    let t33295 = t18866 * t129 * t11799;
    let t33298 = t11798 * t28370 * t7453;
    (t33289, t33291, t33292, t33295, t33298)
}
