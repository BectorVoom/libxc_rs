//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 291/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk291<F: Float>(t1269: F, t225: F, t494: F, t460: F, t487: F, t493: F) -> (F, F, F, F) {
    let t1270 = t1269 * t225;
    let t1271 = t1270 * t494;
    let t1274 = t460 * t487;
    let t1275 = t493 * t493;
    let t1276 = F::new(1.0) / t1275;
    (t1271, t1274, t1275, t1276)
}
