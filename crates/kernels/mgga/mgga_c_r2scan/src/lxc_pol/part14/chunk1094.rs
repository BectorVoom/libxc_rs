//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1094/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1094<F: Float>(t1018: F, t11239: F, t1125: F, t12267: F, t12271: F, t12273: F, t1305: F, t1306: F, t1307: F, t1308: F, t2405: F, t330: F, t3517: F, t3740: F, t3742: F, t41854: F, t41917: F, t837: F, t838: F, t8420: F) -> (F,) {
    let t41940 = (t41854 + t41917) * t330 + 2.0 * t12267 * t837 * t330 + t3740 * t1305 * t330 + t3740 * t1307 * t330 + t11239 * t1018 * t330 + 2.0 * t3517 * t2405 * t330 + 2.0 * t12271 * t838 + t1125 * t8420 * t330 + 2.0 * t12273 * t838 + t3742 * t1306 + t3742 * t1308;
    (t41940,)
}
