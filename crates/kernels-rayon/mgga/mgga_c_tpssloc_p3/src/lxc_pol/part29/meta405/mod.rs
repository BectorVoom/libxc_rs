//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta405 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1651;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1652;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1653;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1654;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta405(t13969: f64, t4988: f64, t1227: f64, t15708: f64, t4723: f64, t11668: f64, t1725: f64, t698: f64, t1174: f64, t1230: f64, t14706: f64, t248: f64, t15426: f64, t68: f64, t484: f64, t11836: f64, t11839: f64, t11842: f64, t15727: f64, t15731: f64, t15735: f64, t15737: f64, t15740: f64, t3490: f64, t3511: f64, t3577: f64, t3580: f64, t3587: f64, t488: f64, t5024: f64, t5030: f64, t15466: f64, t15512: f64, t15558: f64, t15601: f64, t15648: f64, t15684: f64, t15726: f64, t493: f64, t1215: f64, t5052: f64, t1246: f64, t11888: f64, t11904: f64, t11907: f64, t11914: f64, t1201: f64, t1244: f64, t1247: f64, t15032: f64, t15241: f64, t15245: f64, t15248: f64, t15253: f64, t15257: f64, t15430: f64, t1758: f64, t3565: f64, t3604: f64, t3610: f64, t3621: f64, t3624: f64, t3626: f64, t470: f64, t494: f64, t5064: f64, t5069: f64, t5076: f64, t5080: f64, t5084: f64, t5086: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15743, t15745, t15750, t15753, t15754, t15761) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1651(t13969, t4988, t1227, t15708, t4723, t11668, t1725, t698, t1174, t1230, t14706, t248);
        let (t15764, t15768) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1652(t15426, t68, t484, t11836, t11839, t11842, t1227, t15727, t15731, t15735, t15737, t15740, t15745, t15750, t15754, t15761, t3490, t3511, t3577, t3580, t3587, t488, t5024, t5030);
        let (t15771, t15772, t15776, t15777) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1653(t15466, t15512, t15558, t15601, t15648, t15684, t15726, t15768, t493, t1215, t5052, t1246);
        let t15785 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1654(t11888, t11904, t11907, t11914, t1201, t1244, t1247, t15032, t15241, t15245, t15248, t15253, t15257, t15426, t15430, t15772, t15777, t1758, t3565, t3604, t3610, t3621, t3624, t3626, t470, t494, t5064, t5069, t5076, t5080, t5084, t5086);
    (t15743, t15750, t15753, t15761, t15764, t15771, t15776, t15785)
}
