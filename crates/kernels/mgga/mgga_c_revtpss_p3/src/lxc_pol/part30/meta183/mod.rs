//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta183 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk920;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk921;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta183<F: Float>(t3951: F, t547: F, t807: F, t2700: F, t535: F, t1369: F, t794: F, t1372: F, t124: F, t3889: F, t800: F, t2453: F, t546: F, t1389: F, t2713: F, t1414: F, t828: F, t2668: F, t550: F, t816: F, t1379: F, t1408: F, t2482: F, t27: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3952, t3953, t3956, t3957, t3958, t3961, t3964) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk920::<F>(t3951, t547, t807, t2700, t535, t1369, t794, t1372, t124, t3889, t800, t2453, t546);
        let (t3967, t3970, t3974, t3976, t3978) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk921::<F>(t1389, t2713, t3964, t1414, t3889, t828, t2668, t550, t816, t1379, t1408, t2482, t27);
    (t3952, t3953, t3956, t3957, t3958, t3961, t3964, t3967, t3970, t3974, t3976, t3978)
}
