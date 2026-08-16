//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta544 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1981;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1982;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1983;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta544<F: Float>(t1518: F, t7683: F, t1453: F, t1519: F, t2322: F, t27060: F, t28062: F, t28065: F, t28069: F, t28165: F, t28170: F, t28175: F, t28179: F, t29427: F, t29437: F, t4254: F, t569: F, t651: F, t671: F, t8158: F, t8237: F, t2163: F, t4292: F, t670: F, t8233: F, t1911: F, t2165: F, t28183: F, t28186: F, t28188: F, t28190: F, t28192: F, t28193: F, t28201: F, t28202: F, t29432: F, t4248: F, t4257: F, t5787: F, t7586: F, t7591: F, t7687: F, t29343: F, t29425: F, t3: F, t1461: F, t1918: F, t2170: F, t28257: F, t28259: F, t28261: F, t28263: F, t28267: F, t28270: F, t28273: F, t28275: F, t28279: F, t28282: F, t573: F, t5802: F, t5805: F, t7696: F, t8245: F, param_d: F) -> (F, F, F, F, F, F, F) {
        let (t29444, t29451) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1981::<F>(t1518, t7683, t1453, t1519, t2322, t27060, t28062, t28065, t28069, t28165, t28170, t28175, t28179, t29427, t29437, t4254, t569, t651, t671, t8158, t8237);
        let (t29456, t29459, t29466) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1982::<F>(t2163, t4292, t670, t8233, t1519, t1911, t2165, t28183, t28186, t28188, t28190, t28192, t28193, t28201, t28202, t29432, t4248, t4257, t5787, t651, t7586, t7591, t7687);
        let (t29468, t29469, t29480, t29490) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1983::<F>(t29343, t29425, t29451, t29466, t3, t1461, t1918, t2170, t28257, t28259, t28261, t28263, t28267, t28270, t28273, t28275, t28279, t28282, t573, t5802, t5805, t7696, t8245, param_d);
    (t29444, t29456, t29459, t29468, t29469, t29480, t29490)
}
