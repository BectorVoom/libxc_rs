//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta869 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2766;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2767;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta869(t1412: f64, t6861: f64, t2661: f64, t3938: f64, t3992: f64, t5608: f64, t5659: f64, t1399: f64, t22025: f64, t22212: f64, t2496: f64, t1317: f64, t22193: f64, t2626: f64, t1320: f64, t22195: f64, t221: f64, t22253: f64, t4018: f64, t4019: f64, t125: f64, t21969: f64, t6883: f64, t9816: f64, t9818: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t74026, t74029, t74033, t74037, t74106, t74111) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2766(t1412, t6861, t2661, t3938, t3992, t5608, t5659, t1399, t22025, t22212, t2496, t1317, t22193);
        let (t74130, t74132, t74174, t74177, t74184) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2767(t22212, t2626, t1320, t22195, t221, t22253, t4018, t4019, t125, t21969, t1399, t6883, t9816, t9818);
    (t74026, t74029, t74033, t74037, t74106, t74111, t74130, t74132, t74174, t74177, t74184)
}
