//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta845 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2724;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2725;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta845(t3666: f64, t6594: f64, t17283: f64, t5362: f64, t1222: f64, t140: f64, t21209: f64, t21213: f64, t3685: f64, t12865: f64, t5436: f64, t3671: f64, t371: f64, t6609: f64, t676: f64, t1235: f64, t127: f64, t21083: f64, t12967: f64, t20846: f64, t17708: f64, t59550: f64, t12916: f64, t21299: f64, t3718: f64, t20842: f64, t3667: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t70469, t70476, t70491, t70493, t70496, t70511) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2724(t3666, t6594, t17283, t5362, t1222, t140, t21209, t21213, t3685, t12865, t5436, t3671, t371, t6609, t676);
        let (t70521, t70523, t70530, t70542, t70581) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2725(t1235, t127, t21083, t371, t12967, t20846, t17708, t59550, t12916, t21299, t3718, t20842, t3667);
    (t70469, t70476, t70491, t70493, t70496, t70511, t70521, t70523, t70530, t70542, t70581)
}
