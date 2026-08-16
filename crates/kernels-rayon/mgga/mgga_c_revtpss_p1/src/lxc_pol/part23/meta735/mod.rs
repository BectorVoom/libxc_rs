//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta735 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2508;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2509;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta735(t14673: f64, t40731: f64, t40593: f64, t4447: f64, t4462: f64, t10760: f64, t40763: f64, t4353: f64, t1559: f64, t775: f64, t40834: f64, t854: f64, t14587: f64, t2735: f64, t40798: f64, t826: f64, t4452: f64, t14933: f64, t2482: f64, t2668: f64, t2719: f64, t2710: f64, t4371: f64, t9732: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t50605, t50607, t50608, t50611, t50613, t50615) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2508(t14673, t40731, t40593, t4447, t4462, t10760, t40763, t4353, t1559, t775, t40834, t854);
        let (t50619, t50634, t50681, t50703) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2509(t14587, t2735, t40798, t826, t40593, t4452, t14933, t2482, t2668, t2719, t2710, t4371, t9732);
    (t50605, t50607, t50608, t50611, t50613, t50615, t50619, t50634, t50681, t50703)
}
