//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta709 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2464;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2465;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta709<F: Float>(t1904: F, t2439: F, t9640: F, t5718: F, t9292: F, t14274: F, t2435: F, t10175: F, t14090: F, t14085: F, t14104: F, t47520: F, t10069: F, t13731: F, t137: F, t14103: F, t47480: F, t9675: F, t14099: F, t2453: F, t9676: F, t14109: F, t9680: F, t9685: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t47800, t47802, t47806, t47814, t47835, t47837) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2464::<F>(t1904, t2439, t9640, t5718, t9292, t14274, t2435, t10175, t14090, t14085, t14104, t47520);
        let (t47838, t47839, t47845, t47856, t47858, t47860) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2465::<F>(t47837, t10069, t13731, t137, t14103, t47480, t9675, t14099, t2453, t9676, t14109, t9680, t9685);
    (t47800, t47802, t47806, t47814, t47835, t47838, t47839, t47845, t47856, t47858, t47860)
}
