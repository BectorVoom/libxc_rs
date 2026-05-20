//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta371 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1348;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1349;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1350;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1351;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta371<F: Float>(t40196: F, t760: F, t14330: F, t189: F, t2251: F, t2258: F, t10587: F, t2626: F, t2523: F, t9425: F, t2389: F, t37: F, t2612: F, t190: F, t2611: F, t39449: F, t40076: F, t40079: F, t40184: F, t40187: F, t40190: F, t40194: F, t10696: F, t73: F, t2394: F, t2475: F, t2430: F, t10489: F, t10618: F, t10628: F, t10631: F, t10632: F, t10635: F, t14643: F, t225: F, t227: F, t229: F, t2634: F, t2638: F, t2639: F, t2642: F, t39476: F, t39736: F, t39751: F, t39787: F, t40089: F, t40123: F, t40152: F, t40180: F, t4415: F, t830: F, t832: F, t833: F, t231: F, t10639: F, t10657: F, t2754: F, t2815: F, t39707: F, t39712: F, t39714: F, t39719: F, t39723: F, t39724: F, t39726: F, t39731: F, t4514: F, t820: F, t837: F, t879: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t40198, t40202, t40204, t40206, t40207) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1348::<F>(t40196, t760, t14330, t189, t2251, t2258, t10587, t2626, t2523, t9425, t2389, t37);
        let (t40209, t40212, t40213) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1349::<F>(t2612, t40207, t190, t2611, t39449, t40076, t40079, t40184, t40187, t40190, t40194, t40198, t40202, t40204, t40206);
        let (t40232, t40236, t40240, t40250) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1350::<F>(t10696, t73, t2394, t2475, t2430, t10489, t10618, t10628, t10631, t10632, t10635, t14643, t225, t227, t229, t2634, t2638, t2639, t2642, t39476, t39736, t39751, t39787, t40089, t40123, t40152, t40180, t40213, t4415, t830, t832, t833);
        let (t40251, t40255) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1351::<F>(t231, t40250, t10639, t10657, t2754, t2815, t39707, t39712, t39714, t39719, t39723, t39724, t39726, t39731, t4514, t820, t837, t879);
    (t40198, t40202, t40204, t40206, t40209, t40212, t40232, t40236, t40240, t40251, t40255)
}
