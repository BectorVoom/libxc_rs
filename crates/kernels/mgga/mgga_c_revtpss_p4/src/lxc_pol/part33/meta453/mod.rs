//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta453 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1647;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1648;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1649;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1650;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta453<F: Float>(t1248: F, t13045: F, t20956: F, t3720: F, t5341: F, t1219: F, t6667: F, t247: F, t3634: F, t6429: F, t1261: F, t12856: F, t20795: F, t19666: F, t5268: F, t1042: F, t17202: F, t19661: F, t12855: F, t12967: F, t17362: F, t17569: F, t17709: F, t17747: F, t3647: F, t5299: F, t5391: F, t5397: F, t6611: F, t6679: F, t5378: F, t17459: F, t6688: F, t5405: F, t6421: F, t12787: F, t17394: F, t4890: F, t3767: F, t3782: F, t3628: F, t4186: F, t5351: F, t3626: F, t12910: F, t17283: F, t17375: F, t17448: F, t17605: F, t1791: F, t3625: F, t5320: F, t5323: F, t5335: F, t5343: F, t5402: F, t5407: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t20959, t20963, t20966, t20973, t20974, t20977) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1647::<F>(t1248, t13045, t20956, t3720, t5341, t1219, t6667, t247, t3634, t6429, t1261, t12856, t20795);
        let (t20978, t20982, t20986, t20993) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1648::<F>(t20977, t3720, t19666, t5268, t1042, t17202, t19661, t1261, t12855, t12967, t17362, t17569, t17709, t17747, t20959, t20963, t20966, t20974, t3647, t5299, t5391, t5397, t6611, t6679);
        let (t21001, t21004, t21008, t21014, t21017) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1649::<F>(t5378, t5391, t17459, t6688, t3720, t5405, t6421, t12787, t17394, t4890, t3767, t3782);
        let (t21022, t21027) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1650::<F>(t3628, t4186, t5351, t3626, t12910, t17283, t17375, t17448, t17605, t1791, t21001, t21004, t21008, t21014, t21017, t3625, t5320, t5323, t5335, t5343, t5402, t5407);
    (t20959, t20963, t20973, t20978, t20982, t20986, t20993, t21004, t21008, t21022, t21027)
}
