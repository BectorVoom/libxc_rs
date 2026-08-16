//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta404 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1480;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1481;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta404<F: Float>(t3: F, t31204: F, t2198: F, t2327: F, t116: F, t8320: F, t670: F, t2371: F, t8342: F, t117: F, t31157: F, t1459: F, t1461: F, t2207: F, t2209: F, t4158: F, t4162: F, t4165: F, t572: F, t573: F, t8336: F, t8343: F, t8346: F, param_d: F, t10199: F, t655: F, t5787: F, t5517: F, t1312: F, t13426: F, t18227: F, t2199: F, t2201: F, t2322: F, t27123: F, t27126: F, t28219: F, t4248: F, t4254: F, t5523: F, t651: F, t7732: F, t7889: F, t8307: F, t8321: F, t8325: F, t8327: F, t8393: F, t8411: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t31205, t31217, t31231, t31234, t31235, t31238, t31241, t31244) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1480::<F>(t3, t31204, t2198, t2327, t116, t8320, t670, t2371, t8342, t117, t31157, t1459, t1461, t2207, t2209, t4158, t4162, t4165, t572, t573, t8336, t8343, t8346, param_d);
        let (t31287, t31382, t31390, t31398) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1481::<F>(t10199, t655, t2198, t5787, t5517, t1312, t13426, t18227, t2199, t2201, t2322, t27123, t27126, t28219, t4248, t4254, t5523, t651, t7732, t7889, t8307, t8321, t8325, t8327, t8393, t8411);
    (t31205, t31217, t31231, t31234, t31235, t31238, t31241, t31244, t31287, t31382, t31390, t31398)
}
