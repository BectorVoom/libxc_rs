//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 851/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk851<F: Float>(t11589: F, t6: F, t8715: F, t11588: F, t11302: F, t5395: F, t5974: F, t2994: F, t435: F) -> (F, F, F, F, F) {
    let t11591 = t11589 * t6 * t8715;
    let t11592 = t11588 * t11591;
    let t11594 = t5395 * t11302;
    let t11595 = t11594 * t5974;
    let t11597 = t435 * t2994;
    (t11591, t11592, t11594, t11595, t11597)
}
