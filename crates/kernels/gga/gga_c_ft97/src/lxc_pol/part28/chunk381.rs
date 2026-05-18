//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 381/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk381<F: Float>(t5778: F, t5779: F, t28: F, t139: F, t6: F, t1995: F, t1701: F, t538: F, t5546: F, t5551: F, t5555: F) -> (F, F, F, F, F, F) {
    let t5780 = t5778 * t5779;
    let t5781 = t28 * t5780;
    let t5784 = t139 * t6;
    let t5785 = t1995 * t5784;
    let t5787 = t1701 * t5546 * t538;
    let t5790 = t5551 * t5555;
    (t5780, t5781, t5784, t5785, t5787, t5790)
}
