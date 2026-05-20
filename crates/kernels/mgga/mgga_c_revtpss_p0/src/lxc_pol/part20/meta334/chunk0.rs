//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1253/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1253<F: Float>(t12766: F, t13164: F, t1277: F, t13107: F, t225: F, t494: F, t1214: F, t3738: F, t3737: F, t1269: F, t3555: F, t1275: F) -> (F, F, F, F, F, F) {
    let t13165 = t12766 + t13164;
    let t13166 = t1277 * t13165;
    let t13170 = t13107 * t225 * t494;
    let t13173 = t1214 * t3738;
    let t13174 = t3737 * t13173;
    let t13177 = t3555 * t1269;
    let t13180 = t1275 * t1275;
    (t13165, t13166, t13170, t13174, t13177, t13180)
}
