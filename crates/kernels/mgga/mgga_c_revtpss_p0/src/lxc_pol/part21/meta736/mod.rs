//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta736 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2586;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2587;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta736<F: Float>(t47530: F, t9682: F, t2439: F, t3895: F, t4132: F, t1357: F, t689: F, t9659: F, t3899: F, t10175: F, t9671: F, t10146: F, t123: F, t3915: F, t676: F, t10008: F, t1358: F, t212: F, t1359: F, t39501: F, t10115: F, t555: F, t1445: F, t10165: F, t9664: F, t1427: F, t1444: F, t22: F, t9647: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t47531, t47534, t47537, t47540, t47550, t47554) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2586::<F>(t47530, t9682, t2439, t3895, t4132, t1357, t689, t9659, t3899, t10175, t9671, t10146, t123, t3915, t676);
        let (t47558, t47561, t47567, t47568, t47570, t47574) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2587::<F>(t10008, t1358, t212, t689, t1359, t39501, t10115, t555, t1445, t10165, t9664, t1427, t1444, t22, t9647);
    (t47531, t47534, t47537, t47540, t47550, t47554, t47558, t47561, t47567, t47568, t47570, t47574)
}
