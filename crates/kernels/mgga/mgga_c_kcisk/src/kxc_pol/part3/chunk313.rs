//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 313/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk313<F: Float>(t1459: F, t1520: F, t1161: F, t1165: F, t512: F, t507: F, t1184: F, t1176: F, t1181: F, t1188: F, t515: F) -> (F, F, F, F, F, F, F) {
    let t1521 = t1459 * t1520;
    let t1522 = 0.17123333333333333333e-1 * t1161;
    let t1524 = -t1522 - 0.17123333333333333333e-1 * t1165;
    let t1527 = t512 * t512;
    let t1528 = 1.0 / t1527;
    let t1529 = t507 * t1528;
    let t1531 = 0.516475e0 * t1161;
    let t1534 = 0.104195e0 * t1184;
    let t1536 = 0.3529725e1 * t1176 - t1531 - 0.516475e0 * t1165 + 0.6311625e0 * t1181 - t1534 - 0.104195e0 * t1188;
    let t1537 = 1.0 / t515;
    (t1521, t1524, t1527, t1528, t1529, t1536, t1537)
}
