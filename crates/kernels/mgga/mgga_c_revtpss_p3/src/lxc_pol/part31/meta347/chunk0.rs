//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1359/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1359<F: Float>(t828: F, t9954: F, t1398: F, t1868: F, t3935: F, t1882: F, t4003: F, t3957: F, t5690: F, t1873: F, t9741: F, t5651: F, t808: F) -> (F, F, F, F, F, F, F) {
    let t13783 = t9954 * t828;
    let t13784 = t1868 * t1398;
    let t13789 = t3935 * t828;
    let t13790 = t1882 * t4003;
    let t13797 = F::new(7.0) / F::new(72.0) * t3957 * t5690;
    let t13798 = t9741 * t1873;
    let t13800 = t808 * t5651;
    (t13783, t13784, t13789, t13790, t13797, t13798, t13800)
}
