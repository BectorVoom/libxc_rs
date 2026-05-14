//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1158/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1158<F: Float>(t4419: F, t9747: F, t2804: F, t12325: F, t79: F, t2803: F, t33162: F, t12261: F, t2806: F) -> (F, F, F, F, F, F) {
    let t33234 = t4419 * t9747;
    let t33235 = t2804 * t33234;
    let t33257 = t12325 * t79;
    let t33258 = t33257 * t2803;
    let t33270 = t2804 * t33162;
    let t33276 = t12261 * t2806;
    (t33234, t33235, t33257, t33258, t33270, t33276)
}
