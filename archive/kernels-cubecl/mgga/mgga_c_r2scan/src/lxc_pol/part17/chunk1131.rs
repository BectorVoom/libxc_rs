//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1131/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1131<F: Float>(t12025: F, t12027: F, t12030: F, t12034: F, t12037: F, t12039: F, t12040: F, t12046: F, t12048: F, t12049: F, t12053: F, t12055: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t41116 = F::cast_from(45.0_f64) / F::cast_from(32.0_f64) * t12025;
    let t41117 = F::cast_from(5.0_f64) / F::cast_from(8.0_f64) * t12027;
    let t41118 = F::cast_from(5.0_f64) / F::cast_from(8.0_f64) * t12030;
    let t41119 = t12034 / F::cast_from(2.0_f64);
    let t41120 = F::cast_from(5.0_f64) / F::cast_from(8.0_f64) * t12037;
    let t41121 = F::cast_from(2.0_f64) * t12039;
    let t41122 = t12040 / F::cast_from(2.0_f64);
    let t41123 = F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t12046;
    let t41124 = F::cast_from(2.0_f64) * t12048;
    let t41126 = t12049 / F::cast_from(2.0_f64);
    let t41127 = t12053 / F::cast_from(2.0_f64);
    let t41128 = F::cast_from(2.0_f64) * t12055;
    (t41116, t41117, t41118, t41119, t41120, t41121, t41122, t41123, t41124, t41126, t41127, t41128)
}
