//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1838/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1838<F: Float>(t3034: F, t371: F, t1930: F, t6741: F, t3030: F, t3127: F, t363: F, t1011: F, t3040: F, t3131: F, t1014: F, t360: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t23508 = F::cast_from(1.0_f64) / t3034 / t371;
    let t23509 = t1930 * t23508;
    let t23510 = t23509 * t6741;
    let t23511 = t3030 * t3127;
    let t23512 = t23511 * t363;
    let t23513 = t3040 * t1011;
    let t23514 = t23513 * t3131;
    let t23515 = t23512 * t23514;
    let t23518 = t3030 * t1014;
    let t23519 = t23518 * t363;
    let t23520 = t23513 * t360;
    (t23508, t23509, t23510, t23511, t23512, t23514, t23515, t23518, t23519, t23520)
}
