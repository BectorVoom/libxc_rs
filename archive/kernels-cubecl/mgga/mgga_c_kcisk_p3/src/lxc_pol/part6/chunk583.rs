//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 583/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk583<F: Float>(t457: F, t7736: F, t1383: F, t7744: F, t1186: F, t1398: F, t1375: F, t7740: F, t158: F, t165: F, t173: F, t3819: F, t3891: F, t5831: F, t5833: F, t5836: F, t7706: F) -> (F, F, F, F, F, F, F, F) {
    let t8130 = t457 * t7736;
    let t8133 = t1383 * t7744;
    let t8136 = t1186 * t7736;
    let t8139 = t1398 * t7744;
    let t8142 = t1375 * t7736;
    let t8145 = t1375 * t7740;
    let t8148 = t1383 * t7740;
    let t8158 = F::cast_from(0.1171e-2_f64) * t158 * t8130 + F::cast_from(0.7925e-3_f64) * t165 * t8133 - F::cast_from(0.52833333333333333333e-3_f64) * t165 * t8136 + F::cast_from(0.50413125e-5_f64) * t173 * t8139 - F::cast_from(0.672175e-5_f64) * t173 * t8142 + F::cast_from(0.7026e-2_f64) * t158 * t8145 - F::cast_from(0.1585e-2_f64) * t165 * t8148 - F::cast_from(0.23911438650126355246e-1_f64) * t3819 * t7706 + F::cast_from(0.15538616723388920628e-3_f64) * t3891 * t7706 + F::cast_from(0.9368e-2_f64) * t5831 - F::cast_from(0.26416666666666666666e-2_f64) * t5833 - F::cast_from(0.23526125e-4_f64) * t5836;
    (t8130, t8133, t8136, t8139, t8142, t8145, t8148, t8158)
}
