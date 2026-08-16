//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta499 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1983;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1984;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta499(t1211: f64, t20747: f64, t487: f64, t6564: f64, t1770: f64, t1811: f64, t1294: f64, t6744: f64, t3737: f64, t1248: f64, t1715: f64, t3604: f64, t17353: f64, t12712: f64, t6638: f64, t13033: f64, t13058: f64, t17211: f64, t17219: f64, t17227: f64, t17243: f64, t17258: f64, t17260: f64, t17351: f64, t17654: f64, t5270: f64, t5304: f64, t5381: f64, t6631: f64, t6635: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t20748, t20753, t20756, t20759, t20760, t20765, t20766) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1983(t1211, t20747, t487, t6564, t1770, t1811, t1294, t6744, t3737, t1248, t1715, t3604);
        let (t20767, t20770, t20771, t20782) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1984(t17353, t20766, t12712, t6638, t13033, t13058, t17211, t17219, t17227, t17243, t17258, t17260, t17351, t17654, t5270, t5304, t5381, t6631, t6635);
    (t20748, t20753, t20756, t20759, t20760, t20765, t20766, t20767, t20770, t20771, t20782)
}
