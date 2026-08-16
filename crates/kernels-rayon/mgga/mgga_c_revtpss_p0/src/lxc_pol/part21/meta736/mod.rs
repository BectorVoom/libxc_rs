//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta736 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2586;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2587;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta736(t47530: f64, t9682: f64, t2439: f64, t3895: f64, t4132: f64, t1357: f64, t689: f64, t9659: f64, t3899: f64, t10175: f64, t9671: f64, t10146: f64, t123: f64, t3915: f64, t676: f64, t10008: f64, t1358: f64, t212: f64, t1359: f64, t39501: f64, t10115: f64, t555: f64, t1445: f64, t10165: f64, t9664: f64, t1427: f64, t1444: f64, t22: f64, t9647: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t47531, t47534, t47537, t47540, t47550, t47554) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2586(t47530, t9682, t2439, t3895, t4132, t1357, t689, t9659, t3899, t10175, t9671, t10146, t123, t3915, t676);
        let (t47558, t47561, t47567, t47568, t47570, t47574) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2587(t10008, t1358, t212, t689, t1359, t39501, t10115, t555, t1445, t10165, t9664, t1427, t1444, t22, t9647);
    (t47531, t47534, t47537, t47540, t47550, t47554, t47558, t47561, t47567, t47568, t47570, t47574)
}
