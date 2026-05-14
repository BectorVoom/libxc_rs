//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 292/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk292<F: Float>(t1266: F, t492: F, t105: F, t1063: F, t1067: F, t1076: F, t1079: F, t1083: F, t1131: F, t1153: F, t1165: F, t1169: F, t1180: F, t1188: F, t1212: F, t1217: F, t1222: F, t174: F, t419: F, t449: F, t478: F, t489: F) -> (F,) {
    let t1267 = t492 * t1266;
    let t1274 = 0.56910013271352299198e-1 * t1063 * t1067 + t1076 + 0.12646669615856066488e-1 * t1079 * t174 + 0.7588001769513639893e-1 * t1083 * t174 + 0.28455006635676149599e-1 * t1212 * t174 - t1180 - t1188 + t1169 + 0.31616674039640166222e-2 * t1217 * t489 - 0.63233348079280332442e-2 * t1222 * t489 - t1131 + t1165 + t1153 - 0.28455006635676149599e-1 * t105 * t1267 - 0.56910013271352299198e-1 * t419 * t449 + 0.56910013271352299198e-1 * t419 * t478;
    (t1274,)
}
