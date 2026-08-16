//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta405 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1651;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1652;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1653;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1654;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta405<F: Float>(t13969: F, t4988: F, t1227: F, t15708: F, t4723: F, t11668: F, t1725: F, t698: F, t1174: F, t1230: F, t14706: F, t248: F, t15426: F, t68: F, t484: F, t11836: F, t11839: F, t11842: F, t15727: F, t15731: F, t15735: F, t15737: F, t15740: F, t3490: F, t3511: F, t3577: F, t3580: F, t3587: F, t488: F, t5024: F, t5030: F, t15466: F, t15512: F, t15558: F, t15601: F, t15648: F, t15684: F, t15726: F, t493: F, t1215: F, t5052: F, t1246: F, t11888: F, t11904: F, t11907: F, t11914: F, t1201: F, t1244: F, t1247: F, t15032: F, t15241: F, t15245: F, t15248: F, t15253: F, t15257: F, t15430: F, t1758: F, t3565: F, t3604: F, t3610: F, t3621: F, t3624: F, t3626: F, t470: F, t494: F, t5064: F, t5069: F, t5076: F, t5080: F, t5084: F, t5086: F) -> (F, F, F, F, F, F, F, F) {
        let (t15743, t15745, t15750, t15753, t15754, t15761) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1651::<F>(t13969, t4988, t1227, t15708, t4723, t11668, t1725, t698, t1174, t1230, t14706, t248);
        let (t15764, t15768) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1652::<F>(t15426, t68, t484, t11836, t11839, t11842, t1227, t15727, t15731, t15735, t15737, t15740, t15745, t15750, t15754, t15761, t3490, t3511, t3577, t3580, t3587, t488, t5024, t5030);
        let (t15771, t15772, t15776, t15777) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1653::<F>(t15466, t15512, t15558, t15601, t15648, t15684, t15726, t15768, t493, t1215, t5052, t1246);
        let t15785 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1654::<F>(t11888, t11904, t11907, t11914, t1201, t1244, t1247, t15032, t15241, t15245, t15248, t15253, t15257, t15426, t15430, t15772, t15777, t1758, t3565, t3604, t3610, t3621, t3624, t3626, t470, t494, t5064, t5069, t5076, t5080, t5084, t5086);
    (t15743, t15750, t15753, t15761, t15764, t15771, t15776, t15785)
}
