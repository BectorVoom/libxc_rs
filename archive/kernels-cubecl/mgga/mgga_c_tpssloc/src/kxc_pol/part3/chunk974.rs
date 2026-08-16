//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 974/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk974<F: Float>(t3698: F, t3701: F, t112: F, t3931: F, t111: F, t1395: F, t5363: F, t580: F, t1404: F, t1851: F, t5107: F, t671: F) -> (F, F, F, F, F, F) {
    let t12477 = t3698 * t3701;
    let t12521 = t3931 * t112;
    let t12524 = t1395 * t111;
    let t12541 = F::cast_from(2.0_f64) * t5363 * t580;
    let t12543 = F::cast_from(2.0_f64) * t1851 * t1404;
    let t12545 = t5107 * t671;
    (t12477, t12521, t12524, t12541, t12543, t12545)
}
