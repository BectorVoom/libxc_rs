//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 982/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk982<F: Float>(t12884: F, t247: F, t3363: F, t1261: F, t12690: F, t225: F, t480: F, t1231: F, t3655: F, t1256: F, t3651: F, t2434: F, t371: F, t482: F, t481: F, t3172: F, t3605: F) -> (F, F, F, F, F, F) {
    let t12886 = t247 * t12884 * t3363;
    let t12887 = t1261 * t12886;
    let t12889 = t12690 * t225;
    let t12890 = t12889 * t480;
    let t12893 = t1231 * t3655;
    let t12895 = t3651 * t1256;
    let t12898 = t371 * t2434 * t482;
    let t12900 = 0.63517063878621832551e-4 * t481 * t12898;
    let t12901 = t3172 * t3605;
    (t12887, t12890, t12893, t12895, t12900, t12901)
}
