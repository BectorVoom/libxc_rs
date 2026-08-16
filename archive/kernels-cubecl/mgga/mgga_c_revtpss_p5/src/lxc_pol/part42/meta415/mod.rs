//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta415 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1471;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1472;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta415<F: Float>(t2195: F, t2289: F, t31027: F, t8312: F, t31032: F, t8316: F, t104: F, t2357: F, t116: F, t8320: F, t10199: F, t655: F, t2198: F, t5787: F, t5517: F, t1312: F, t13426: F, t18227: F, t2199: F, t2201: F, t2322: F, t27123: F, t27126: F, t28219: F, t4248: F, t4254: F, t5523: F, t651: F, t7732: F, t7889: F, t8307: F, t8321: F, t8325: F, t8327: F, t8393: F, t8411: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t31134, t31135, t31137, t31149, t31234, t31287) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1471::<F>(t2195, t2289, t31027, t8312, t31032, t8316, t104, t2357, t116, t8320, t10199, t655);
        let (t31382, t31390, t31398) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1472::<F>(t2198, t5787, t5517, t1312, t13426, t18227, t2199, t2201, t2322, t27123, t27126, t28219, t4248, t4254, t5523, t651, t7732, t7889, t8307, t8321, t8325, t8327, t8393, t8411);
    (t31134, t31135, t31137, t31149, t31234, t31287, t31382, t31390, t31398)
}
