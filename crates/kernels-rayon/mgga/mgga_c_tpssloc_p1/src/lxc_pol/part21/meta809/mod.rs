//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta809 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2828;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2829;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2830;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2831;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2832;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2833;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2834;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta809(t16558: f64, t2770: f64, t607: f64, t123: f64, t2768: f64, t17177: f64, t2250: f64, t55723: f64, t17188: f64, t690: f64, t55677: f64, t883: f64, t882: f64, t41658: f64, t41675: f64, t41684: f64, t59655: f64, t59657: f64, t59661: f64, t59663: f64, t59665: f64, t2394: f64, t5682: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t59668, t59670) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2828(t16558, t2770, t607, t123, t2768);
        let (t59672, t59674) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2829(t17177, t2250, t123, t2768);
        let (t59676, t59678) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2830(t2770, t55723, t123, t2768);
        let t59680 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2831(t17188, t690);
        let (t59682, t59684) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2832(t55677, t883, t123, t882);
        let t59686 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2833(t41658, t41675, t41684, t59655, t59657, t59661, t59663, t59665, t59670, t59674, t59678, t59680, t59684);
        let t59688 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2834(t2394, t5682);
    (t59668, t59670, t59672, t59674, t59676, t59678, t59680, t59682, t59684, t59686, t59688)
}
