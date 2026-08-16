//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3471/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3471<F: Float>(t63665: F, t63668: F, t63670: F, t63673: F, t63676: F, t63679: F, t63681: F, t63683: F, t63685: F, t63820: F, t63826: F, t63833: F, t63835: F) -> F {
    let t65392 = -t63665 - t63668 - t63670 - t63673 - t63676 - t63679 + t63681 - t63683 + t63685 - t63820 + t63826 - t63833 - t63835;
    t65392
}
