//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta510 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2004;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2005;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta510(t20977: f64, t3720: f64, t19666: f64, t5268: f64, t1042: f64, t17202: f64, t19661: f64, t1261: f64, t12855: f64, t12967: f64, t17362: f64, t17569: f64, t17709: f64, t17747: f64, t20959: f64, t20963: f64, t20966: f64, t20974: f64, t3647: f64, t5299: f64, t5391: f64, t5397: f64, t6611: f64, t6679: f64, t5378: f64, t17459: f64, t6688: f64, t5405: f64, t6421: f64, t12787: f64, t17394: f64, t4890: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t20978, t20981, t20982, t20985, t20986, t20993) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2004(t20977, t3720, t19666, t5268, t1042, t17202, t19661, t1261, t12855, t12967, t17362, t17569, t17709, t17747, t20959, t20963, t20966, t20974, t3647, t5299, t5391, t5397, t6611, t6679);
        let (t21001, t21003, t21004, t21007, t21008, t21013) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2005(t5378, t5391, t17459, t6688, t3720, t5405, t6421, t12787, t17394, t4890);
    (t20978, t20981, t20982, t20985, t20986, t20993, t21001, t21003, t21004, t21007, t21008, t21013)
}
