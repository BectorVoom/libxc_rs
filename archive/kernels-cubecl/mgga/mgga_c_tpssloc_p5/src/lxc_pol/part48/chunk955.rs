//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 955/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk955<F: Float>(t114726: F, t114740: F, t23035: F, t2379: F, t31376: F, t6637: F, t114674: F, t1888: F, t232: F, t6646: F, t31386: F, t6579: F) -> (F, F, F, F) {
    let t114741 = t114726 + t114740;
    let t114746 = t23035 * t6637 * t31376 * t2379;
    let t114750 = t1888 * t6646 * t114674 * t232;
    let t114752 = t6579 * t31386;
    (t114741, t114746, t114750, t114752)
}
