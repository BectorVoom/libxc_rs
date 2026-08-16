//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta389 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1831;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1832;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1833;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1834;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta389(t1248: f64, t3568: f64, t1287: f64, t1269: f64, t1284: f64, t1209: f64, t3584: f64, t12233: f64, t12240: f64, t12242: f64, t12245: f64, t12251: f64, t12360: f64, t12363: f64, t12573: f64, t12575: f64, t12577: f64, t12598: f64, t12224: f64, t12237: f64, t12366: f64, t12381: f64, t12395: f64, t12413: f64, t12417: f64, t12561: f64, t12566: f64, t12579: f64, t12583: f64, t12594: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t12718, t12719, t12722, t12723) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1831(t1248, t3568, t1287, t1269, t1284, t1209);
        let (t12726, t12727, t12730) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1832(t1248, t3584, t1287, t12233, t12240, t12242, t12245, t12251, t12360, t12363, t12573, t12575, t12577, t12598);
        let t12731 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1833(t12224, t12237, t12366, t12381, t12395, t12413, t12417, t12561, t12566, t12579, t12583, t12594);
        let t12732 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1834(t12730, t12731);
    (t12718, t12719, t12722, t12723, t12726, t12727, t12732)
}
