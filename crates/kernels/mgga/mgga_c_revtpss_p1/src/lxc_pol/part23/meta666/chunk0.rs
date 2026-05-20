//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2397/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2397<F: Float>(t1021: F, t11970: F, t11874: F, t15688: F, t11853: F, t828: F, t3181: F, t675: F, t283: F, t2852: F, t11144: F, t3252: F) -> (F, F, F, F, F, F) {
    let t42326 = t1021 * t11970;
    let t42328 = t11874 * t15688;
    let t42410 = t828 * t11853;
    let t42447 = t675 * t3181;
    let t42471 = F::new(1.0) / t283 / t2852;
    let t42518 = t3252 * t11144;
    (t42326, t42328, t42410, t42447, t42471, t42518)
}
