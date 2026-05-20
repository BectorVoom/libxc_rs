//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta393 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1844;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1845;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta393<F: Float>(t12810: F, t5352: F, t3720: F, t12269: F, t247: F, t3618: F, t12277: F, t1264: F, t12273: F, t1284: F, t3555: F, t3624: F, t12803: F, t3629: F, t3626: F, t1121: F, t3603: F, t606: F, t1222: F, t1261: F, t1266: F, t12774: F, t12777: F, t12781: F, t12784: F, t12789: F, t12794: F, t12797: F, t12800: F, t12805: F, t12809: F, t3620: F, t3625: F, t3631: F, t3640: F, t3644: F, t3647: F, t3718: F, t3723: F, t5340: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t12811, t12812, t12816, t12822, t12828, t12831, t12832) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1844::<F>(t12810, t5352, t3720, t12269, t247, t3618, t12277, t1264, t12273, t1284, t3555, t3624);
        let (t12835, t12836, t12841, t12842, t12845) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1845::<F>(t12803, t3629, t3626, t1121, t3603, t606, t12810, t1222, t1261, t1266, t12774, t12777, t12781, t12784, t12789, t12794, t12797, t12800, t12805, t12809, t12812, t12816, t12822, t12828, t12832, t3620, t3625, t3631, t3640, t3644, t3647, t3718, t3723, t5340);
    (t12811, t12812, t12816, t12822, t12828, t12831, t12832, t12835, t12836, t12841, t12842, t12845)
}
