//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1131/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1131<F: Float>(t1450: F, t1868: F, t1873: F, t7252: F, t1885: F, t7264: F, t1889: F, t7271: F, t1892: F, t1955: F, t2047: F, t7719: F) -> (F, F, F, F, F, F) {
    let t7900 = t1450 * t1868;
    let t7904 = t7252 * t1873;
    let t7906 = t7264 * t1885;
    let t7908 = t7271 * t1889;
    let t7917 = t1955 * t1892;
    let t7964 = t2047 * t7719;
    (t7900, t7904, t7906, t7908, t7917, t7964)
}
