//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta735 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2508;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2509;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta735<F: Float>(t14673: F, t40731: F, t40593: F, t4447: F, t4462: F, t10760: F, t40763: F, t4353: F, t1559: F, t775: F, t40834: F, t854: F, t14587: F, t2735: F, t40798: F, t826: F, t4452: F, t14933: F, t2482: F, t2668: F, t2719: F, t2710: F, t4371: F, t9732: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t50605, t50607, t50608, t50611, t50613, t50615) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2508::<F>(t14673, t40731, t40593, t4447, t4462, t10760, t40763, t4353, t1559, t775, t40834, t854);
        let (t50619, t50634, t50681, t50703) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2509::<F>(t14587, t2735, t40798, t826, t40593, t4452, t14933, t2482, t2668, t2719, t2710, t4371, t9732);
    (t50605, t50607, t50608, t50611, t50613, t50615, t50619, t50634, t50681, t50703)
}
