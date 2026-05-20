//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1757/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1757<F: Float>(t1927: F, t644: F, t4144: F, t9593: F, t2034: F, t2014: F, t10416: F, t1937: F, t13435: F, t2322: F, t6993: F, t196: F, t197: F, t3821: F) -> (F, F, F, F, F, F, F, F) {
    let t25163 = t1927 * t644;
    let t25177 = t9593 * t4144;
    let t25178 = t2034 * t25177;
    let t25180 = F::new(2.0) * t2014 * t25178;
    let t25182 = F::new(2.0) * t10416 * t1937;
    let t25184 = F::new(4.0) * t13435 * t1937;
    let t25186 = F::new(4.0) * t2322 * t6993;
    let t25188 = t3821 * t196 * t197;
    (t25163, t25177, t25178, t25180, t25182, t25184, t25186, t25188)
}
