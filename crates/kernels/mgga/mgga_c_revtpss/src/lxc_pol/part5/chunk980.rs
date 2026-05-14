//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 980/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk980<F: Float>(t1398: F, t1868: F, t3935: F, t828: F, t1882: F, t4003: F, t3957: F, t5690: F, t1873: F, t9741: F, t5651: F, t808: F, t9736: F, t241: F, t820: F, t9991: F) -> (F, F, F, F, F, F, F) {
    let t13784 = t1868 * t1398;
    let t13789 = t3935 * t828;
    let t13790 = t1882 * t4003;
    let t13797 = 7.0 / 72.0 * t3957 * t5690;
    let t13798 = t9741 * t1873;
    let t13800 = t808 * t5651;
    let t13801 = t9736 * t13800;
    let t13804 = t820 * t9991 * t241;
    (t13784, t13789, t13790, t13797, t13798, t13801, t13804)
}
