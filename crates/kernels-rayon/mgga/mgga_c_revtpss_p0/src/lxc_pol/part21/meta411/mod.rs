//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta411 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1879;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1880;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta411(t13147: f64, t460: f64, t12051: f64, t13045: f64, t13128: f64, t13111: f64, t3783: f64, t3568: f64, t3759: f64, t12629: f64, t1280: f64, t1204: f64, t1234: f64, t12769: f64, t1281: f64, t1285: f64, t12966: f64, t12975: f64, t12987: f64, t13108: f64, t13112: f64, t13118: f64, t13121: f64, t13127: f64, t13130: f64, t13134: f64, t13142: f64, t13144: f64, t3666: f64, t3670: f64, t3746: f64, t3751: f64, t3760: f64, t3763: f64, t3767: f64, t3778: f64, t3782: f64, t3787: f64, t12766: f64, t1277: f64, t13107: f64, t225: f64, t494: f64, t1214: f64, t3738: f64, t3737: f64, t1269: f64, t3555: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13148, t13149, t13150, t13153, t13156, t13161, t13164) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1879(t13147, t460, t12051, t13045, t13128, t13111, t3783, t3568, t3759, t12629, t1280, t1204, t1234, t12769, t1281, t1285, t12966, t12975, t12987, t13108, t13112, t13118, t13121, t13127, t13130, t13134, t13142, t13144, t3666, t3670, t3746, t3751, t3760, t3763, t3767, t3778, t3782, t3787);
        let (t13165, t13166, t13170, t13173, t13174, t13177) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1880(t12766, t13164, t1277, t13107, t225, t494, t1214, t3738, t3737, t1269, t3555);
    (t13148, t13149, t13150, t13153, t13156, t13161, t13165, t13166, t13170, t13173, t13174, t13177)
}
