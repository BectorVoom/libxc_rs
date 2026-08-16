//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta389 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1831;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1832;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1833;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1834;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta389<F: Float>(t1248: F, t3568: F, t1287: F, t1269: F, t1284: F, t1209: F, t3584: F, t12233: F, t12240: F, t12242: F, t12245: F, t12251: F, t12360: F, t12363: F, t12573: F, t12575: F, t12577: F, t12598: F, t12224: F, t12237: F, t12366: F, t12381: F, t12395: F, t12413: F, t12417: F, t12561: F, t12566: F, t12579: F, t12583: F, t12594: F) -> (F, F, F, F, F, F, F) {
        let (t12718, t12719, t12722, t12723) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1831::<F>(t1248, t3568, t1287, t1269, t1284, t1209);
        let (t12726, t12727, t12730) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1832::<F>(t1248, t3584, t1287, t12233, t12240, t12242, t12245, t12251, t12360, t12363, t12573, t12575, t12577, t12598);
        let t12731 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1833::<F>(t12224, t12237, t12366, t12381, t12395, t12413, t12417, t12561, t12566, t12579, t12583, t12594);
        let t12732 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1834::<F>(t12730, t12731);
    (t12718, t12719, t12722, t12723, t12726, t12727, t12732)
}
