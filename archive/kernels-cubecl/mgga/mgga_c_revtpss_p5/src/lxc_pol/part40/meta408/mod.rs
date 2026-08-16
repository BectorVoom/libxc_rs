//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta408 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1487;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1488;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta408<F: Float>(t31398: F, t31461: F, t3: F, t2198: F, t670: F, t1518: F, t31234: F, t4292: F, t8342: F, t116: F, t8406: F, t117: F, t31451: F, param_d: F, t1459: F, t1461: F, t1916: F, t1918: F, t2207: F, t2209: F, t572: F, t573: F, t5795: F, t5802: F, t5805: F, t8336: F, t8343: F, t8346: F, t8421: F, t8427: F, t8430: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t31463, t31464, t31475, t31493, t31494, t31497, t31500, t31505, t31506, t31509) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1487::<F>(t31398, t31461, t3, t2198, t670, t1518, t31234, t4292, t8342, t116, t8406, t117, t31451, param_d);
        let t31512 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1488::<F>(t1459, t1461, t1916, t1918, t2207, t2209, t31475, t31494, t31497, t31500, t31506, t31509, t572, t573, t5795, t5802, t5805, t8336, t8343, t8346, t8421, t8427, t8430);
    (t31463, t31464, t31475, t31493, t31494, t31497, t31500, t31505, t31506, t31509, t31512)
}
