//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta805 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2636;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2637;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta805(t1568: f64, t4423: f64, t2783: f64, t6041: f64, t786: f64, t2801: f64, t231: f64, t2782: f64, t18689: f64, t2435: f64, t18688: f64, t2439: f64, t2777: f64, t14587: f64, t51548: f64, t14602: f64, t14961: f64, t1558: f64, t2482: f64, t4469: f64, t14520: f64, t14568: f64, t14524: f64, t51297: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t62808, t62809, t62840, t62843, t62847) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2636(t1568, t4423, t2783, t6041, t786, t2801, t231, t2782, t18689, t2435, t18688, t2439, t2777);
        let (t62853, t62866, t62868, t62872, t62874) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2637(t14587, t2782, t51548, t14602, t14961, t1558, t2482, t4469, t14520, t14568, t14524, t51297);
    (t62808, t62809, t62840, t62843, t62847, t62853, t62866, t62868, t62872, t62874)
}
