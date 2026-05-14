//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1136/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1136<F: Float>(t27475: F, t303: F, t5628: F, t1014: F, t28473: F, t12147: F, t28438: F, t7908: F, t16685: F, t27369: F, t27396: F, t27416: F, t27453: F, t28480: F, t3984: F, t52402: F, t5709: F, t7901: F, t7909: F, t8144: F, t94651: F, t98081: F, t98087: F) -> (F, F, F) {
    let t98883 = t303 * t27475 * t5628;
    let t98887 = t1014 * t28473;
    let t98888 = 0.33163888888888888888e-2 * t98887;
    let t98903 = 0.15445601851851851852e-3 * t7908 * t12147 * t28438;
    let t98906 = 0.69505208333333333333e-3 * t8144 * t27416 + 0.30891203703703703704e-3 * t94651 - 0.49745833333333333332e-2 * t98883 - 0.37069444444444444444e-2 * t28480 * t7901 + t98888 + 0.23168402777777777778e-3 * t7908 * t3984 * t7909 * t52402 + 0.23168402777777777778e-3 * t7908 * t98081 - 0.92754700520833333335e-4 * t27369 * t98087 + 0.46336805555555555556e-3 * t7908 * t5709 * t27453 * t16685 + t98903 - 0.13901041666666666667e-2 * t8144 * t27396;
    (t98883, t98887, t98906)
}
