//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta358 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1712;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1713;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1714;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta358(t11659: f64, t4910: f64, t3117: f64, t1016: f64, t697: f64, t1011: f64, t1010: f64, t2270: f64, t3241: f64, t3244: f64, t1058: f64, t3197: f64, t11132: f64, t11134: f64, t11136: f64, t11138: f64, t11140: f64, t11147: f64, t11153: f64, t11158: f64, t11162: f64, t11167: f64, t11171: f64, t341: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11876, t11877, t11880, t11881, t11883) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1712(t11659, t4910, t3117, t1016, t697, t1011, t1010, t2270);
        let (t11886, t11888, t11890, t11901) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1713(t3241, t3244, t1058, t3197, t11132, t11134, t11136, t11138, t11140, t11147, t11153, t11158, t11162, t11167, t11171);
        let t11902 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1714(t11901, t341);
    (t11876, t11877, t11880, t11881, t11883, t11886, t11888, t11890, t11901, t11902)
}
