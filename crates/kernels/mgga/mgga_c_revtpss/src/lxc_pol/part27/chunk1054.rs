//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1054/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1054<F: Float>(t1261: F, t12886: F, t12690: F, t225: F, t480: F, t1231: F, t3655: F, t1256: F, t3651: F, t2434: F, t371: F, t482: F) -> (F, F, F, F, F, F) {
    let t12887 = t1261 * t12886;
    let t12889 = t12690 * t225;
    let t12890 = t12889 * t480;
    let t12893 = t1231 * t3655;
    let t12895 = t3651 * t1256;
    let t12898 = t371 * t2434 * t482;
    (t12887, t12889, t12890, t12893, t12895, t12898)
}
