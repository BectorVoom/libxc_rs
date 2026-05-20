//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta805 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2636;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2637;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta805<F: Float>(t1568: F, t4423: F, t2783: F, t6041: F, t786: F, t2801: F, t231: F, t2782: F, t18689: F, t2435: F, t18688: F, t2439: F, t2777: F, t14587: F, t51548: F, t14602: F, t14961: F, t1558: F, t2482: F, t4469: F, t14520: F, t14568: F, t14524: F, t51297: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t62808, t62809, t62840, t62843, t62847) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2636::<F>(t1568, t4423, t2783, t6041, t786, t2801, t231, t2782, t18689, t2435, t18688, t2439, t2777);
        let (t62853, t62866, t62868, t62872, t62874) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2637::<F>(t14587, t2782, t51548, t14602, t14961, t1558, t2482, t4469, t14520, t14568, t14524, t51297);
    (t62808, t62809, t62840, t62843, t62847, t62853, t62866, t62868, t62872, t62874)
}
