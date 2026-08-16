//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta273 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1127;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1128;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta273<F: Float>(t1583: F, t30: F, t1544: F, t33: F, t1518: F, t93: F, t1847: F, t196: F, t197: F, t1450: F, t1868: F, t1873: F, t7252: F, t1885: F, t7264: F, t1889: F, t7271: F, t1892: F, t1955: F, t2047: F, t7719: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t7787, t7862, t7869, t7889, t7897, t7898) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1127::<F>(t1583, t30, t1544, t33, t1518, t93, t1847, t196, t197);
        let (t7900, t7904, t7906, t7908, t7917, t7964) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1128::<F>(t1450, t1868, t1873, t7252, t1885, t7264, t1889, t7271, t1892, t1955, t2047, t7719);
    (t7787, t7862, t7869, t7889, t7897, t7898, t7900, t7904, t7906, t7908, t7917, t7964)
}
