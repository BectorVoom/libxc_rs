//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta386 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1444;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1445;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1446;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta386(t1353: f64, t5591: f64, t4012: f64, t828: f64, t1868: f64, t3889: f64, t221: f64, t5627: f64, t9921: f64, t3978: f64, t13583: f64, t13585: f64, t13593: f64, t13599: f64, t13612: f64, t13615: f64, t9278: f64, t9308: f64, t9316: f64, t9320: f64, t9325: f64, t9329: f64, t9333: f64, t9374: f64, t9389: f64, t9391: f64, t13620: f64, t13622: f64, t13623: f64, t13624: f64, t13629: f64, t13631: f64, t13633: f64, t13634: f64, t13635: f64, t13636: f64, t13637: f64, t9394: f64, t9415: f64, t9421: f64, t9427: f64, t9546: f64, t13640: f64, t13641: f64, t13643: f64, t13644: f64, t13645: f64, t13646: f64, t13647: f64, t13653: f64, t13655: f64, t9514: f64, t9517: f64, t9521: f64, t9555: f64, t9569: f64, t9574: f64, t9577: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13867, t13869, t13872, t13874, t13877, t13878, t13880, t13881) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1444(t1353, t5591, t4012, t828, t1868, t3889, t221, t5627, t9921, t3978, t13583, t13585, t13593, t13599, t13612, t13615, t9278, t9308, t9316, t9320, t9325, t9329, t9333, t9374, t9389, t9391);
        let t13882 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1445(t13620, t13622, t13623, t13624, t13629, t13631, t13633, t13634, t13635, t13636, t13637, t9394, t9415, t9421, t9427, t9546);
        let t13884 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1446(t13640, t13641, t13643, t13644, t13645, t13646, t13647, t13653, t13655, t9514, t9517, t9521, t9555, t9569, t9574, t9577);
    (t13867, t13869, t13872, t13874, t13877, t13878, t13880, t13881, t13882, t13884)
}
