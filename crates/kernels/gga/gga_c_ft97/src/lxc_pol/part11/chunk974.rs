//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 974/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk974<F: Float>(t10866: F, t8675: F, t10871: F, t10847: F, t10877: F, t10862: F, t2925: F, t39448: F, t10859: F, t2938: F, t703: F, t10855: F, t10845: F, t10864: F, t2265: F, t2405: F, t2409: F, t2413: F, t2923: F, t2939: F, t2951: F, t904: F, t9578: F) -> (F,) {
    let t43146 = t8675 * t10866;
    let t43148 = t8675 * t10871;
    let t43150 = t8675 * t10847;
    let t43152 = t8675 * t10877;
    let t43158 = t8675 * t10862;
    let t43160 = t39448 * t2925;
    let t43162 = t8675 * t10859;
    let t43164 = t703 * t2938;
    let t43177 = t8675 * t10855;
    let t43183 = -2.0 * t2265 * t2923 * t2413 * t2951 - 8.0 * t43146 - 16.0 / 3.0 * t43148 + 8.0 / 9.0 * t43150 + 8.0 / 3.0 * t43152 - 2.0 / 3.0 * t2265 * t10845 * t2405 * t2951 + 8.0 / 3.0 * t43158 - 40.0 / 9.0 * t43160 + 8.0 / 3.0 * t43162 + 2.0 * t2265 * t43164 * t2405 * t2939 + 8.0 / 3.0 * t2265 * t10845 * t9578 * t904 + 4.0 * t2265 * t2923 * t2409 * t2951 - 4.0 / 9.0 * t43177 + 6.0 * t2265 * t10864 * t2413 * t2939;
    (t43183,)
}
