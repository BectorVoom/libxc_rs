//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2692/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2692<F: Float>(t4772: F, t4975: F, t19462: F, t3286: F, t3298: F, t6235: F, t3316: F, t1086: F, t19856: F, t16543: F, t4746: F, t1647: F, t16551: F) -> (F, F, F, F, F, F, F) {
    let t67668 = t4975 * t4772;
    let t67714 = t19462 * t3286;
    let t67725 = t6235 * t3298;
    let t67790 = t6235 * t3316;
    let t67825 = t19856 * t1086;
    let t67927 = t4746 * t16543;
    let t67969 = t1647 * t16551;
    (t67668, t67714, t67725, t67790, t67825, t67927, t67969)
}
