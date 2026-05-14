//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 828/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk828<F: Float>(t1163: F, t1248: F, t13614: F, t13522: F, t13526: F, t13530: F, t13533: F, t13536: F, t13540: F, t13543: F, t13546: F, t13549: F, t13552: F, t13555: F, t1235: F, t344: F) -> (F, F, F, F) {
    let t13616 = t1248 * t13614 * t1163;
    let t13618 = 28.0 / 27.0 * t13522;
    let t13629 = -t13618 - 4.0 / 9.0 * t13526 + 2.0 / 9.0 * t13530 - 2.0 / 3.0 * t13533 + t13536 / 3.0 - 10.0 / 27.0 * t13540 + 4.0 / 3.0 * t13543 - 2.0 / 3.0 * t13546 - 2.0 * t13549 + 2.0 * t13552 - t13555 / 3.0;
    let t13630 = t1235 * t13629;
    let t13632 = 1.0/pow_3_2(t344);
    (t13616, t13629, t13630, t13632)
}
