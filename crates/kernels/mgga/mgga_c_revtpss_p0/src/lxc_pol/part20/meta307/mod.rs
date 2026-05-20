//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta307 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1202;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1203;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta307<F: Float>(t1196: F, t12581: F, t1298: F, t3798: F, t3800: F, t498: F, t12487: F, t12552: F, t12555: F, t1188: F, t3520: F, t1294: F, t3568: F, t1277: F, t1204: F, t1269: F, t3584: F, t12295: F, t12292: F, t12297: F, t12299: F, t12301: F, t12303: F, t12307: F, t12310: F, t12314: F, t12317: F, t12320: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t12583, t12584, t12587, t12592, t12594, t12596, t12598, t12599) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1202::<F>(t1196, t12581, t1298, t3798, t3800, t498, t12487, t12552, t12555, t1188, t3520, t1294, t3568);
        let (t12600, t12603, t12607, t12621) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1203::<F>(t12599, t1277, t1204, t1269, t1294, t3584, t12295, t12292, t12297, t12299, t12301, t12303, t12307, t12310, t12314, t12317, t12320);
    (t12583, t12584, t12587, t12592, t12594, t12596, t12598, t12600, t12603, t12607, t12621)
}
