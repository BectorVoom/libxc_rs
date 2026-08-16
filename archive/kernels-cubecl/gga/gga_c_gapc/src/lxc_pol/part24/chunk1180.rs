//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1180/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1180<F: Float>(t11326: F, t21642: F, t9260: F, t20563: F, t5116: F, t9061: F, t3709: F, t3713: F, t5075: F, t11450: F, t11451: F, t21157: F) -> (F, F, F, F) {
    let t34695 = t11326 * t9260 * t21642;
    let t34698 = t9061 * t5116 * t20563;
    let t34701 = t3709 * t5075 * t3713;
    let t34704 = t11450 * t11451 * t21157;
    (t34695, t34698, t34701, t34704)
}
