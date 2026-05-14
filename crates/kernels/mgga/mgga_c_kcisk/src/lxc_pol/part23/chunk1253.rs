//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1253/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1253<F: Float>(t4163: F, t4169: F, t14293: F, t1455: F, t14292: F, t475: F, t503: F, t15092: F, t551: F, t554: F, t15093: F, t1607: F, t4528: F, t4534: F, t4349: F, t14608: F, t1553: F) -> (F, F, F, F, F, F, F, F) {
    let t41204 = t4163 * t4169;
    let t41209 = t1455 * t14293;
    let t41218 = t475 / t14292 / t503;
    let t41849 = t551 / t15092 / t554;
    let t41861 = t1607 * t15093;
    let t41864 = t4528 * t4534;
    let t42126 = t4349 * t4349;
    let t42127 = 1.0 / t42126;
    let t42942 = t1553 * t14608;
    (t41204, t41209, t41218, t41849, t41861, t41864, t42127, t42942)
}
