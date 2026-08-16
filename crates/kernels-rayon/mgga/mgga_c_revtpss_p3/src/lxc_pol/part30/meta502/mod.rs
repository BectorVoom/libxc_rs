//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta502 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1872;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1873;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1874;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta502(t2142: f64, t3568: f64, t7637: f64, t3584: f64, t3551: f64, t1204: f64, t2144: f64, t26886: f64, t26889: f64, t26891: f64, t26895: f64, t26897: f64, t26901: f64, t26906: f64, t26909: f64, t26913: f64, t26918: f64, t26922: f64, t26924: f64, t26928: f64, t26933: f64, t26937: f64, t26941: f64, t26945: f64, t26949: f64, t3552: f64, t3791: f64, t460: f64, t7629: f64, t7632: f64, t7636: f64, t7643: f64, t7651: f64, t7654: f64, t7659: f64, t7662: f64, t13181: f64, t473: f64, t3738: f64, t3566: f64, t26936: f64, t7642: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t26951, t26959, t26963, t26968) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1872(t2142, t3568, t7637, t3584, t3551, t1204, t2144, t26886, t26889, t26891, t26895, t26897, t26901, t26906, t26909, t26913, t26918, t26922, t26924, t26928, t26933, t26937, t26941, t26945, t26949, t3552, t3791, t460, t7629, t7632, t7636, t7643, t7651, t7654, t7659, t7662);
        let t26969 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1873(t13181, t473);
        let (t26971, t26976, t26979) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1874(t2142, t3738, t26969, t3566, t26936, t7642);
    (t26951, t26959, t26963, t26968, t26969, t26971, t26976, t26979)
}
