//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1194/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1194<F: Float>(t34329: F, t9705: F, t1800: F, t7413: F, t1950: F, t6974: F, t15851: F, t748: F, t2587: F, t4581: F, t1873: F, t7327: F, t2580: F, t4817: F, t34314: F, t34317: F, t34319: F, t34322: F, t34325: F, t34327: F) -> (F, F, F, F, F, F, F, F) {
    let t34330 = t34329 * t9705;
    let t34332 = t1800 * t7413;
    let t34334 = t6974 * t1950;
    let t34336 = t15851 * t748;
    let t34338 = t4581 * t2587;
    let t34340 = t1873 * t7327;
    let t34342 = t4817 * t2580;
    let t34344 = -t34314 / 16.0 + t34317 / 24.0 + t34319 / 96.0 - t34322 / 288.0 - t34325 / 16.0 + t34327 / 24.0 + t34330 / 6.0 + t34332 / 18.0 + t34334 / 128.0 - t34336 / 128.0 + t34338 / 24.0 - t34340 / 24.0 + t34342 / 128.0;
    (t34330, t34332, t34334, t34336, t34338, t34340, t34342, t34344)
}
