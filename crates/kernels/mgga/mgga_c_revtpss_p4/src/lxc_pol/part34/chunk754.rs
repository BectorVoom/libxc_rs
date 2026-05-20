//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 754/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk754<F: Float>(t2035: F, t7898: F, t1450: F, t1868: F, t7237: F, t2014: F, t1873: F, t7252: F, t1885: F, t7264: F, t1889: F, t7271: F) -> (F, F, F, F, F, F, F) {
    let t7899 = t7898 * t2035;
    let t7900 = t1450 * t1868;
    let t7901 = t7237 * t7900;
    let t7903 = F::new(3.0) * t2014 * t7901;
    let t7904 = t7252 * t1873;
    let t7906 = t7264 * t1885;
    let t7908 = t7271 * t1889;
    (t7899, t7900, t7901, t7903, t7904, t7906, t7908)
}
