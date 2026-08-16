//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta453 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1647;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1648;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1649;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1650;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta453(t1248: f64, t13045: f64, t20956: f64, t3720: f64, t5341: f64, t1219: f64, t6667: f64, t247: f64, t3634: f64, t6429: f64, t1261: f64, t12856: f64, t20795: f64, t19666: f64, t5268: f64, t1042: f64, t17202: f64, t19661: f64, t12855: f64, t12967: f64, t17362: f64, t17569: f64, t17709: f64, t17747: f64, t3647: f64, t5299: f64, t5391: f64, t5397: f64, t6611: f64, t6679: f64, t5378: f64, t17459: f64, t6688: f64, t5405: f64, t6421: f64, t12787: f64, t17394: f64, t4890: f64, t3767: f64, t3782: f64, t3628: f64, t4186: f64, t5351: f64, t3626: f64, t12910: f64, t17283: f64, t17375: f64, t17448: f64, t17605: f64, t1791: f64, t3625: f64, t5320: f64, t5323: f64, t5335: f64, t5343: f64, t5402: f64, t5407: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t20959, t20963, t20966, t20973, t20974, t20977) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1647(t1248, t13045, t20956, t3720, t5341, t1219, t6667, t247, t3634, t6429, t1261, t12856, t20795);
        let (t20978, t20982, t20986, t20993) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1648(t20977, t3720, t19666, t5268, t1042, t17202, t19661, t1261, t12855, t12967, t17362, t17569, t17709, t17747, t20959, t20963, t20966, t20974, t3647, t5299, t5391, t5397, t6611, t6679);
        let (t21001, t21004, t21008, t21014, t21017) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1649(t5378, t5391, t17459, t6688, t3720, t5405, t6421, t12787, t17394, t4890, t3767, t3782);
        let (t21022, t21027) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1650(t3628, t4186, t5351, t3626, t12910, t17283, t17375, t17448, t17605, t1791, t21001, t21004, t21008, t21014, t21017, t3625, t5320, t5323, t5335, t5343, t5402, t5407);
    (t20959, t20963, t20973, t20978, t20982, t20986, t20993, t21004, t21008, t21022, t21027)
}
