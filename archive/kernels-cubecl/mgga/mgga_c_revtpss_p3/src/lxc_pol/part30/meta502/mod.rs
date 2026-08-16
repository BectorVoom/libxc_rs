//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta502 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1872;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1873;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1874;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta502<F: Float>(t2142: F, t3568: F, t7637: F, t3584: F, t3551: F, t1204: F, t2144: F, t26886: F, t26889: F, t26891: F, t26895: F, t26897: F, t26901: F, t26906: F, t26909: F, t26913: F, t26918: F, t26922: F, t26924: F, t26928: F, t26933: F, t26937: F, t26941: F, t26945: F, t26949: F, t3552: F, t3791: F, t460: F, t7629: F, t7632: F, t7636: F, t7643: F, t7651: F, t7654: F, t7659: F, t7662: F, t13181: F, t473: F, t3738: F, t3566: F, t26936: F, t7642: F) -> (F, F, F, F, F, F, F, F) {
        let (t26951, t26959, t26963, t26968) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1872::<F>(t2142, t3568, t7637, t3584, t3551, t1204, t2144, t26886, t26889, t26891, t26895, t26897, t26901, t26906, t26909, t26913, t26918, t26922, t26924, t26928, t26933, t26937, t26941, t26945, t26949, t3552, t3791, t460, t7629, t7632, t7636, t7643, t7651, t7654, t7659, t7662);
        let t26969 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1873::<F>(t13181, t473);
        let (t26971, t26976, t26979) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1874::<F>(t2142, t3738, t26969, t3566, t26936, t7642);
    (t26951, t26959, t26963, t26968, t26969, t26971, t26976, t26979)
}
