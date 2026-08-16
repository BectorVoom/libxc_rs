//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta371 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1348;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1349;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1350;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1351;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta371(t40196: f64, t760: f64, t14330: f64, t189: f64, t2251: f64, t2258: f64, t10587: f64, t2626: f64, t2523: f64, t9425: f64, t2389: f64, t37: f64, t2612: f64, t190: f64, t2611: f64, t39449: f64, t40076: f64, t40079: f64, t40184: f64, t40187: f64, t40190: f64, t40194: f64, t10696: f64, t73: f64, t2394: f64, t2475: f64, t2430: f64, t10489: f64, t10618: f64, t10628: f64, t10631: f64, t10632: f64, t10635: f64, t14643: f64, t225: f64, t227: f64, t229: f64, t2634: f64, t2638: f64, t2639: f64, t2642: f64, t39476: f64, t39736: f64, t39751: f64, t39787: f64, t40089: f64, t40123: f64, t40152: f64, t40180: f64, t4415: f64, t830: f64, t832: f64, t833: f64, t231: f64, t10639: f64, t10657: f64, t2754: f64, t2815: f64, t39707: f64, t39712: f64, t39714: f64, t39719: f64, t39723: f64, t39724: f64, t39726: f64, t39731: f64, t4514: f64, t820: f64, t837: f64, t879: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40198, t40202, t40204, t40206, t40207) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1348(t40196, t760, t14330, t189, t2251, t2258, t10587, t2626, t2523, t9425, t2389, t37);
        let (t40209, t40212, t40213) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1349(t2612, t40207, t190, t2611, t39449, t40076, t40079, t40184, t40187, t40190, t40194, t40198, t40202, t40204, t40206);
        let (t40232, t40236, t40240, t40250) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1350(t10696, t73, t2394, t2475, t2430, t10489, t10618, t10628, t10631, t10632, t10635, t14643, t225, t227, t229, t2634, t2638, t2639, t2642, t39476, t39736, t39751, t39787, t40089, t40123, t40152, t40180, t40213, t4415, t830, t832, t833);
        let (t40251, t40255) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1351(t231, t40250, t10639, t10657, t2754, t2815, t39707, t39712, t39714, t39719, t39723, t39724, t39726, t39731, t4514, t820, t837, t879);
    (t40198, t40202, t40204, t40206, t40209, t40212, t40232, t40236, t40240, t40251, t40255)
}
