//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1847/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1847<F: Float>(t1888: F, t232: F, t6646: F, t87106: F, t1484: F, t852: F, t25038: F, t25248: F, t776: F, t13393: F, t22996: F, t22986: F, t25249: F, t2633: F) -> (F, F, F, F, F) {
    let t87109 = t1888 * t6646 * t87106 * t232;
    let t87111 = t852 * t1484;
    let t87114 = t25038 * t25248 * t87111 * t776;
    let t87117 = t1888 * t22996 * t13393;
    let t87124 = t22986 * t22996 * t25249 * t2633;
    (t87109, t87111, t87114, t87117, t87124)
}
