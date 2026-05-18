//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1396/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1396<F: Float>(t30835: F, t34672: F, t34675: F, t34678: F, t34681: F, t34684: F, t34687: F, t34691: F, t34699: F, t34701: F, t34702: F, t34706: F, t34709: F, t34712: F, t34714: F, t34717: F) -> F {
    let t38664 = -t34672 + t34675 - t34678 + t34681 + t34684 + t34687 + t34691 - t34699 + t34701 + t30835 + t34702 - t34706 - t34709 - t34712 + t34714 + t34717;
    t38664
}
