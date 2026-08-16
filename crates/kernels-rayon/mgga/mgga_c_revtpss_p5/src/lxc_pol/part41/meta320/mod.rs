//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta320 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1098;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1099;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta320(t5686: f64, t9744: f64, t221: f64, t4019: f64, t5659: f64, t4018: f64, t3989: f64, t5629: f64, t3930: f64, t5661: f64, t5665: f64, t9976: f64, t1412: f64, t1882: f64, t3938: f64, t3992: f64, t2661: f64, t1399: f64, t5608: f64, t5651: f64, t5774: f64, t72: f64, t686: f64, t3915: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14024, t14038, t14040, t14042, t14043) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1098(t5686, t9744, t221, t4019, t5659, t4018, t3989, t5629, t3930, t5661, t5665, t9976);
        let (t14045, t14049, t14053, t14057, t14081) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1099(t1412, t1882, t3938, t3992, t2661, t1399, t5608, t5651, t5774, t72, t686, t3915);
    (t14024, t14038, t14040, t14042, t14043, t14045, t14049, t14053, t14057, t14081)
}
