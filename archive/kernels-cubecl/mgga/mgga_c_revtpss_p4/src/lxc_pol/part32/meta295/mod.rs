//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta295 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1197;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1198;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta295<F: Float>(t2453: F, t4100: F, t1398: F, t281: F, t543: F, t68: F, t10115: F, t562: F, t2435: F, t3903: F, t1445: F, t3895: F, t2439: F, t1420: F, t3908: F, t1426: F, t786: F, t64: F, t843: F, t112: F, t2289: F, t666: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t10139, t10143, t10157, t10160, t10162) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1197::<F>(t2453, t4100, t1398, t281, t543, t68, t10115, t562, t2435, t3903, t1445, t3895);
        let (t10163, t10166, t10175, t10199, t10201, t10202) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1198::<F>(t10162, t2439, t1420, t2453, t3908, t1426, t786, t64, t843, t112, t2289, t666);
    (t10139, t10143, t10157, t10160, t10163, t10166, t10175, t10199, t10201, t10202)
}
