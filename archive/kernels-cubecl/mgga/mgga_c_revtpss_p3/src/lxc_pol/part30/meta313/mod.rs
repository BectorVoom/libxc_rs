//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta313 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1307;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta313<F: Float>(t4067: F, t786: F, t1364: F, t213: F, t4066: F, t1420: F, t1426: F, t3917: F, t64: F, t843: F, t112: F, t2289: F, t666: F) -> (F, F, F, F, F, F, F) {
        let (t10169, t10171, t10175, t10176, t10199, t10201, t10202) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1307::<F>(t4067, t786, t1364, t213, t4066, t1420, t1426, t3917, t64, t843, t112, t2289, t666);
    (t10169, t10171, t10175, t10176, t10199, t10201, t10202)
}
