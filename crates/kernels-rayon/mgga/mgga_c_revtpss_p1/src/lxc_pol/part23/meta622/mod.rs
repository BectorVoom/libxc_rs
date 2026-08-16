//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta622 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2304;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2305;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2306;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2307;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta622(t1250: f64, t24751: f64, t3720: f64, t1715: f64, t20809: f64, t1042: f64, t5192: f64, t6548: f64, t12552: f64, t24375: f64, t12555: f64, t1196: f64, t24255: f64, t24257: f64, t24259: f64, t24261: f64, t24482: f64, t24484: f64, t24490: f64, t24496: f64, t24500: f64, t24214: f64, t24217: f64, t24219: f64, t24223: f64, t24264: f64, t24326: f64, t24329: f64, t24468: f64, t24472: f64, t24475: f64, t24478: f64, t24492: f64, t482: f64, t1247: f64, t1261: f64, t12866: f64, t12910: f64, t17396: f64, t17401: f64, t17505: f64, t1797: f64, t21107: f64, t21252: f64, t21255: f64, t24726: f64, t24731: f64, t24736: f64, t24741: f64, t24744: f64, t3711: f64, t3718: f64, t5331: f64, t5340: f64, t6619: f64, t6690: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t24752, t24753, t24758, t24759, t24763, t24764, t24765, t24767) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2304(t1250, t24751, t3720, t1715, t20809, t1042, t5192, t6548, t12552, t24375, t12555, t1196);
        let (t24768, t24769) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2305(t24255, t24257, t24259, t24261, t24482, t24484, t24490, t24496, t24500, t24763, t24767, t24214, t24217, t24219, t24223, t24264, t24326, t24329, t24468, t24472, t24475, t24478, t24492);
        let t24770 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2306(t24768, t24769);
        let (t24772, t24773, t24778) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2307(t1250, t24770, t482, t1042, t1247, t1261, t12866, t12910, t17396, t17401, t17505, t1797, t21107, t21252, t21255, t24726, t24731, t24736, t24741, t24744, t24753, t24759, t3711, t3718, t5331, t5340, t6619, t6690);
    (t24752, t24753, t24758, t24759, t24763, t24764, t24765, t24767, t24770, t24772, t24773, t24778)
}
