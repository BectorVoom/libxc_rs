//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 881/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk881<F: Float>(t2617: F, t2696: F, t2693: F, t809: F, t597: F, t61: F, t241: F, t244: F, t248: F, t238: F, t154: F, t9569: F) -> (F, F, F, F, F) {
    let t9993 = t2617 * t2696;
    let t10014 = t809 * t2693;
    let t10021 = F::cast_from(1.0_f64) / t61 / t597;
    let t10022 = t10021 * t241;
    let t10024 = t10022 * t244 * t248;
    let t10026 = F::cast_from(595.0_f64) / F::cast_from(10368.0_f64) * t238 * t10024;
    let t10027 = t9569 * t154;
    (t9993, t10014, t10022, t10026, t10027)
}
