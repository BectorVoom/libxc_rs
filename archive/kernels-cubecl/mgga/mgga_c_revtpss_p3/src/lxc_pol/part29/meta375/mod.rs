//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta375 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1341;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1342;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1343;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta375<F: Float>(t1353: F, t5591: F, t4012: F, t828: F, t1868: F, t3889: F, t221: F, t5627: F, t9921: F, t3978: F, t13583: F, t13585: F, t13593: F, t13599: F, t13612: F, t13615: F, t9278: F, t9308: F, t9316: F, t9320: F, t9325: F, t9329: F, t9333: F, t9374: F, t9389: F, t9391: F, t13620: F, t13622: F, t13623: F, t13624: F, t13629: F, t13631: F, t13633: F, t13634: F, t13635: F, t13636: F, t13637: F, t9394: F, t9415: F, t9421: F, t9427: F, t9546: F, t13640: F, t13641: F, t13643: F, t13644: F, t13645: F, t13646: F, t13647: F, t13653: F, t13655: F, t9514: F, t9517: F, t9521: F, t9555: F, t9569: F, t9574: F, t9577: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t13867, t13869, t13872, t13874, t13877, t13878, t13880, t13881) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1341::<F>(t1353, t5591, t4012, t828, t1868, t3889, t221, t5627, t9921, t3978, t13583, t13585, t13593, t13599, t13612, t13615, t9278, t9308, t9316, t9320, t9325, t9329, t9333, t9374, t9389, t9391);
        let t13882 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1342::<F>(t13620, t13622, t13623, t13624, t13629, t13631, t13633, t13634, t13635, t13636, t13637, t9394, t9415, t9421, t9427, t9546);
        let t13884 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1343::<F>(t13640, t13641, t13643, t13644, t13645, t13646, t13647, t13653, t13655, t9514, t9517, t9521, t9555, t9569, t9574, t9577);
    (t13867, t13869, t13872, t13874, t13877, t13878, t13880, t13881, t13882, t13884)
}
