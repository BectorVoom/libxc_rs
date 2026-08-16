//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta429 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1615;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1616;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1617;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta429(t12800: f64, t3636: f64, t3551: f64, t3565: f64, t225: f64, t480: f64, t12884: f64, t828: f64, t12788: f64, t3625: f64, t12732: f64, t73: f64, t13039: f64, t44372: f64, t44373: f64, t13045: f64, t42871: f64, t3597: f64, t3603: f64, t3367: f64, t2251: f64, t12839: f64, t2258: f64, t1042: f64, t1261: f64, t12784: f64, t12803: f64, t12810: f64, t12836: f64, t12842: f64, t13100: f64, t17426: f64, t17638: f64, t17644: f64, t247: f64, t3610: f64, t3611: f64, t3626: f64, t3629: f64, t3674: f64, t43777: f64, t44333: f64, t44377: f64, t5340: f64, t3555: f64, t3766: f64, t5330: f64, t1209: f64, t13147: f64, t17708: f64, t11249: f64, t13043: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t44418, t44420, t44421, t44422, t44427, t44431) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1615(t12800, t3636, t3551, t3565, t225, t480, t12884, t828, t12788, t3625, t12732, t73);
        let t44479 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1616(t13039, t44372, t44373, t13045, t42871, t3597, t3603, t3367, t2251, t12839, t2258, t1042, t1261, t12784, t12803, t12810, t12836, t12842, t13100, t17426, t17638, t17644, t247, t3610, t3611, t3625, t3626, t3629, t3674, t43777, t44333, t44377, t44418, t44422, t44427, t44431, t5340);
        let (t44484, t44500, t44501) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1617(t3555, t3766, t5330, t1209, t13147, t17708, t11249, t13043);
    (t44420, t44421, t44431, t44479, t44484, t44500, t44501)
}
