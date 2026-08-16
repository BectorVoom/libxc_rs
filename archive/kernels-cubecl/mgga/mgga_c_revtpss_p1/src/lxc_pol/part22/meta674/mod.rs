//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta674 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2650;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2651;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta674<F: Float>(t1284: F, t6564: F, t6688: F, t73: F, t5458: F, t1287: F, t21257: F, t1811: F, t3766: F, t460: F, t3781: F, t21040: F, t12702: F, t12717: F, t12744: F, t1285: F, t1288: F, t17307: F, t17958: F, t21416: F, t21427: F, t21430: F, t21436: F, t3666: F, t3670: F, t3755: F, t3767: F, t3782: F, t5326: F, t5436: F, t5443: F, t5446: F, t5466: F, t5470: F, t5481: F, t5487: F, t6720: F, t6727: F, t6738: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t21439, t21442, t21443, t21448, t21451, t21452, t21455, t21456, t21459) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2650::<F>(t1284, t6564, t6688, t73, t5458, t1287, t21257, t1811, t3766, t460, t3781, t21040);
        let t21464 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2651::<F>(t12702, t12717, t12744, t1285, t1288, t17307, t17958, t21416, t21427, t21430, t21436, t21439, t21443, t21448, t21452, t21456, t21459, t3666, t3670, t3755, t3767, t3782, t5326, t5436, t5443, t5446, t5466, t5470, t5481, t5487, t6720, t6727, t6738);
    (t21439, t21442, t21443, t21448, t21451, t21452, t21455, t21456, t21459, t21464)
}
