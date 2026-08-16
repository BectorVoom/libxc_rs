//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1505/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1505<F: Float>(t1558: F, t5962: F, t10777: F, t14671: F, t14686: F, t6017: F, t10811: F, t23293: F, t1544: F, t23327: F, t23323: F, t14586: F, t14931: F, t61715: F) -> (F, F, F, F, F, F, F) {
    let t76302 = t5962 * t1558;
    let t76313 = t10777 * t14686 * t14671 * t6017;
    let t76315 = t10811 * t23293;
    let t76321 = t1544 * t1558;
    let t76330 = t10811 * t23327;
    let t76337 = t10811 * t23323;
    let t76362 = t14931 * t14686 * t61715 * t14586;
    (t76302, t76313, t76315, t76321, t76330, t76337, t76362)
}
