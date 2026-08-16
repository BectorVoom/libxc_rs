//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta451 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2003;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2004;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2005;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta451<F: Float>(t13969: F, t4988: F, t1227: F, t15708: F, t4723: F, t11668: F, t1725: F, t698: F, t1174: F, t1230: F, t14706: F, t248: F, t15426: F, t68: F, t484: F, t11836: F, t11839: F, t11842: F, t15727: F, t15731: F, t15735: F, t15737: F, t15740: F, t3490: F, t3511: F, t3577: F, t3580: F, t3587: F, t488: F, t5024: F, t5030: F, t15466: F, t15512: F, t15558: F, t15601: F, t15648: F, t15684: F, t15726: F, t493: F, t1215: F, t5052: F, t1246: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t15743, t15745, t15749, t15750, t15753, t15754, t15761) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2003::<F>(t13969, t4988, t1227, t15708, t4723, t11668, t1725, t698, t1174, t1230, t14706, t248);
        let (t15764, t15765, t15768) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2004::<F>(t15426, t68, t484, t11836, t11839, t11842, t1227, t15727, t15731, t15735, t15737, t15740, t15745, t15750, t15754, t15761, t3490, t3511, t3577, t3580, t3587, t488, t5024, t5030);
        let (t15771, t15772, t15777) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2005::<F>(t15466, t15512, t15558, t15601, t15648, t15684, t15726, t15768, t493, t1215, t5052, t1246);
    (t15743, t15745, t15749, t15750, t15753, t15754, t15761, t15764, t15765, t15771, t15772, t15777)
}
