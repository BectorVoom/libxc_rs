//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 759/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk759<F: Float>(t3121: F, t9050: F, t1625: F, t1720: F, t8987: F, t197: F, t4991: F, t1022: F, t3: F, t5: F, t8785: F, t8784: F) -> (F, F, F, F, F, F, F) {
    let t9051 = t3121 * t9050;
    let t9053 = t1720 * t1625;
    let t9054 = t8987 * t9053;
    let t9056 = t197 * t4991;
    let t9057 = t1022 * t9056;
    let t9059 = t3 * t5;
    let t9060 = t9059 * t8785;
    let t9061 = t8784 * t9060;
    (t9051, t9053, t9054, t9057, t9059, t9060, t9061)
}
