//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1043/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1043<F: Float>(t1060: F, t269: F, t783: F, t9083: F, t12550: F, t788: F, t37616: F, t37630: F, t37634: F, t37639: F, t39500: F, t39503: F, t39512: F, t39523: F, t41405: F, t43072: F) -> (F,) {
    let t43076 = t783 * t9083 * t269 * t1060;
    let t43079 = t783 * t12550 * t788;
    let t43081 = t41405 - 0.42377972951376424087e0 * t37616 - 0.59512461497092438715e-1 * t37630 - 0.17853738449127731614e0 * t37634 - 0.14457274399185490173e-3 * t37639 - 0.26198215989259945075e-1 * t43072 + t39500 - t39503 + t39512 - 0.21831846657716620896e-2 * t43076 + 0.23287303101564395623e-1 * t43079 + t39523;
    (t43081,)
}
