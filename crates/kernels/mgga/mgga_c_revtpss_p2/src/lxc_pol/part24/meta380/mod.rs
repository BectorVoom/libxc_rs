//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta380 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1275;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1276;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1277;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1278;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1279;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta380<F: Float>(t1794: F, t6587: F, t1250: F, t3720: F, t1715: F, t20809: F, t1042: F, t5192: F, t6548: F, t12552: F, t24375: F, t12555: F, t1196: F, t24255: F, t24257: F, t24259: F, t24261: F, t24482: F, t24484: F, t24490: F, t24496: F, t24500: F, t24214: F, t24217: F, t24219: F, t24223: F, t24264: F, t24326: F, t24329: F, t24468: F, t24472: F, t24475: F, t24478: F, t24492: F, t482: F, t1247: F, t1261: F, t12866: F, t12910: F, t17396: F, t17401: F, t17505: F, t1797: F, t21107: F, t21252: F, t21255: F, t24726: F, t24731: F, t24736: F, t24741: F, t24744: F, t3711: F, t3718: F, t5331: F, t5340: F, t6619: F, t6690: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t24751, t24752, t24753, t24758, t24759, t24763, t24765) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1275::<F>(t1794, t6587, t1250, t3720, t1715, t20809, t1042, t5192, t6548, t12552, t24375, t12555);
        let (t24767, t24768) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1276::<F>(t1196, t24765, t24255, t24257, t24259, t24261, t24482, t24484, t24490, t24496, t24500, t24763);
        let t24769 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1277::<F>(t24214, t24217, t24219, t24223, t24264, t24326, t24329, t24468, t24472, t24475, t24478, t24492);
        let t24770 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1278::<F>(t24768, t24769);
        let (t24772, t24773, t24778) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1279::<F>(t1250, t24770, t482, t1042, t1247, t1261, t12866, t12910, t17396, t17401, t17505, t1797, t21107, t21252, t21255, t24726, t24731, t24736, t24741, t24744, t24753, t24759, t3711, t3718, t5331, t5340, t6619, t6690);
    (t24751, t24752, t24753, t24758, t24759, t24763, t24765, t24767, t24770, t24772, t24773, t24778)
}
