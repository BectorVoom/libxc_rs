//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta451 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2003;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2004;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2005;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta451(t13969: f64, t4988: f64, t1227: f64, t15708: f64, t4723: f64, t11668: f64, t1725: f64, t698: f64, t1174: f64, t1230: f64, t14706: f64, t248: f64, t15426: f64, t68: f64, t484: f64, t11836: f64, t11839: f64, t11842: f64, t15727: f64, t15731: f64, t15735: f64, t15737: f64, t15740: f64, t3490: f64, t3511: f64, t3577: f64, t3580: f64, t3587: f64, t488: f64, t5024: f64, t5030: f64, t15466: f64, t15512: f64, t15558: f64, t15601: f64, t15648: f64, t15684: f64, t15726: f64, t493: f64, t1215: f64, t5052: f64, t1246: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15743, t15745, t15749, t15750, t15753, t15754, t15761) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2003(t13969, t4988, t1227, t15708, t4723, t11668, t1725, t698, t1174, t1230, t14706, t248);
        let (t15764, t15765, t15768) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2004(t15426, t68, t484, t11836, t11839, t11842, t1227, t15727, t15731, t15735, t15737, t15740, t15745, t15750, t15754, t15761, t3490, t3511, t3577, t3580, t3587, t488, t5024, t5030);
        let (t15771, t15772, t15777) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2005(t15466, t15512, t15558, t15601, t15648, t15684, t15726, t15768, t493, t1215, t5052, t1246);
    (t15743, t15745, t15749, t15750, t15753, t15754, t15761, t15764, t15765, t15771, t15772, t15777)
}
