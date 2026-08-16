//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta307 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1202;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1203;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta307(t1196: f64, t12581: f64, t1298: f64, t3798: f64, t3800: f64, t498: f64, t12487: f64, t12552: f64, t12555: f64, t1188: f64, t3520: f64, t1294: f64, t3568: f64, t1277: f64, t1204: f64, t1269: f64, t3584: f64, t12295: f64, t12292: f64, t12297: f64, t12299: f64, t12301: f64, t12303: f64, t12307: f64, t12310: f64, t12314: f64, t12317: f64, t12320: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12583, t12584, t12587, t12592, t12594, t12596, t12598, t12599) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1202(t1196, t12581, t1298, t3798, t3800, t498, t12487, t12552, t12555, t1188, t3520, t1294, t3568);
        let (t12600, t12603, t12607, t12621) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1203(t12599, t1277, t1204, t1269, t1294, t3584, t12295, t12292, t12297, t12299, t12301, t12303, t12307, t12310, t12314, t12317, t12320);
    (t12583, t12584, t12587, t12592, t12594, t12596, t12598, t12600, t12603, t12607, t12621)
}
