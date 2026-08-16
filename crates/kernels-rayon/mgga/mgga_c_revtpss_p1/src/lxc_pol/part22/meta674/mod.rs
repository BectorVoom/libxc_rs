//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta674 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2650;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2651;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta674(t1284: f64, t6564: f64, t6688: f64, t73: f64, t5458: f64, t1287: f64, t21257: f64, t1811: f64, t3766: f64, t460: f64, t3781: f64, t21040: f64, t12702: f64, t12717: f64, t12744: f64, t1285: f64, t1288: f64, t17307: f64, t17958: f64, t21416: f64, t21427: f64, t21430: f64, t21436: f64, t3666: f64, t3670: f64, t3755: f64, t3767: f64, t3782: f64, t5326: f64, t5436: f64, t5443: f64, t5446: f64, t5466: f64, t5470: f64, t5481: f64, t5487: f64, t6720: f64, t6727: f64, t6738: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t21439, t21442, t21443, t21448, t21451, t21452, t21455, t21456, t21459) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2650(t1284, t6564, t6688, t73, t5458, t1287, t21257, t1811, t3766, t460, t3781, t21040);
        let t21464 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2651(t12702, t12717, t12744, t1285, t1288, t17307, t17958, t21416, t21427, t21430, t21436, t21439, t21443, t21448, t21452, t21456, t21459, t3666, t3670, t3755, t3767, t3782, t5326, t5436, t5443, t5446, t5466, t5470, t5481, t5487, t6720, t6727, t6738);
    (t21439, t21442, t21443, t21448, t21451, t21452, t21455, t21456, t21459, t21464)
}
