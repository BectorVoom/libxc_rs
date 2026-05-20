//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta429 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1615;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1616;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1617;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta429<F: Float>(t12800: F, t3636: F, t3551: F, t3565: F, t225: F, t480: F, t12884: F, t828: F, t12788: F, t3625: F, t12732: F, t73: F, t13039: F, t44372: F, t44373: F, t13045: F, t42871: F, t3597: F, t3603: F, t3367: F, t2251: F, t12839: F, t2258: F, t1042: F, t1261: F, t12784: F, t12803: F, t12810: F, t12836: F, t12842: F, t13100: F, t17426: F, t17638: F, t17644: F, t247: F, t3610: F, t3611: F, t3626: F, t3629: F, t3674: F, t43777: F, t44333: F, t44377: F, t5340: F, t3555: F, t3766: F, t5330: F, t1209: F, t13147: F, t17708: F, t11249: F, t13043: F) -> (F, F, F, F, F, F, F) {
        let (t44418, t44420, t44421, t44422, t44427, t44431) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1615::<F>(t12800, t3636, t3551, t3565, t225, t480, t12884, t828, t12788, t3625, t12732, t73);
        let t44479 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1616::<F>(t13039, t44372, t44373, t13045, t42871, t3597, t3603, t3367, t2251, t12839, t2258, t1042, t1261, t12784, t12803, t12810, t12836, t12842, t13100, t17426, t17638, t17644, t247, t3610, t3611, t3625, t3626, t3629, t3674, t43777, t44333, t44377, t44418, t44422, t44427, t44431, t5340);
        let (t44484, t44500, t44501) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1617::<F>(t3555, t3766, t5330, t1209, t13147, t17708, t11249, t13043);
    (t44420, t44421, t44431, t44479, t44484, t44500, t44501)
}
