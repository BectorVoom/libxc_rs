//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1113/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1113<F: Float>(t14895: F, t8801: F, t14188: F, t26958: F, t353: F, t4228: F, t814: F, t859: F, t14888: F, t19906: F, t15034: F, t892: F, t1161: F, t52191: F, t53952: F, t27729: F, t4082: F) -> (F, F, F, F, F, F, F, F) {
    let t55672 = 7.0 / 24.0 * t8801 * t14895;
    let t55695 = 7.0 / 72.0 * t26958 * t14188;
    let t55698 = t859 * t353 * t4228 * t814;
    let t55702 = 7.0 / 72.0 * t19906 * t14888;
    let t55717 = t859 * t892 * t15034;
    let t55722 = t859 * t353 * t52191 * t1161;
    let t55726 = 7.0 / 144.0 * t53952;
    let t55729 = t27729 * t4082;
    (t55672, t55695, t55698, t55702, t55717, t55722, t55726, t55729)
}
