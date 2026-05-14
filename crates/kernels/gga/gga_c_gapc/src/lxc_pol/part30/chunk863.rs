//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 863/1135 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk863<F: Float>(t11772: F, t9799: F, t11522: F, t7451: F, t9396: F, t11479: F, t2660: F, t2767: F, t1084: F, t11387: F) -> (F, F, F, F, F) {
    let t11773 = t11772 * t9799;
    let t11775 = t7451 * t11522;
    let t11776 = t11775 * t9396;
    let t11779 = t2660 * t11479 * t2767;
    let t11781 = t1084 * t11387;
    (t11773, t11775, t11776, t11779, t11781)
}
