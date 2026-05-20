//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta320 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1098;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1099;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta320<F: Float>(t5686: F, t9744: F, t221: F, t4019: F, t5659: F, t4018: F, t3989: F, t5629: F, t3930: F, t5661: F, t5665: F, t9976: F, t1412: F, t1882: F, t3938: F, t3992: F, t2661: F, t1399: F, t5608: F, t5651: F, t5774: F, t72: F, t686: F, t3915: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t14024, t14038, t14040, t14042, t14043) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1098::<F>(t5686, t9744, t221, t4019, t5659, t4018, t3989, t5629, t3930, t5661, t5665, t9976);
        let (t14045, t14049, t14053, t14057, t14081) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1099::<F>(t1412, t1882, t3938, t3992, t2661, t1399, t5608, t5651, t5774, t72, t686, t3915);
    (t14024, t14038, t14040, t14042, t14043, t14045, t14049, t14053, t14057, t14081)
}
